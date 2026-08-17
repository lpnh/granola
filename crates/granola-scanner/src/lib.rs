use std::collections::{HashMap, HashSet, hash_map::Entry};
use std::fs;
use std::io;
use std::path::Path;

use ignore::WalkBuilder;
use ignore::types::TypesBuilder;
use proc_macro2::{TokenStream, TokenTree};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::{Expr, Item, Stmt, Token, UseTree};

mod catalog;

use catalog::COMPONENTS;
pub use catalog::write_daisyui_safelist;
use granola::daisyui::Component;

const GRANOLA: &str = "granola";
const DAISYUI: &str = "daisyui";

fn find_component(name: &str) -> Option<&'static Component> {
    COMPONENTS.iter().find(|c| {
        c.module == name || c.type_name == name || c.parts.iter().any(|p| p.type_name == name)
    })
}

fn class_for_component_path(path: &[&str]) -> Option<&'static str> {
    match path {
        [module, rest @ ..] => {
            if let Some(component) = find_component(module)
                && let Some(class) = component.class_for_path(rest)
            {
                return Some(class);
            }
            COMPONENTS.iter().find_map(|c| c.class_for_path(path))
        }
        _ => None,
    }
}

fn class_for_macro(name: &str) -> Option<&'static str> {
    COMPONENTS
        .iter()
        .flat_map(|c| {
            std::iter::once((c.macro_name, c.base_class))
                .chain(c.parts.iter().map(|p| (p.macro_name, p.class_name)))
        })
        .find_map(|(macro_name, class_name)| (macro_name == name).then_some(class_name))
}

/// Scans one Rust source file or the Rust source files below a directory.
pub fn scan_dir(root: impl AsRef<Path>) -> io::Result<HashSet<String>> {
    let root = root.as_ref();
    let mut files = Vec::new();
    walk_rs_files(root, &mut |path| {
        let source = fs::read_to_string(path)?;
        match syn::parse_file(&source) {
            Ok(file) => files.push((module_path(root, path).unwrap_or_default(), file)),
            Err(_) => println!(
                "cargo:warning=granola-scanner: could not parse {}",
                path.display()
            ),
        }
        Ok(())
    })?;

    let mut module_exports = HashMap::new();
    let mut glob_exports = Vec::new();
    for (module, file) in &files {
        collect_module_exports(&file.items, module, &mut module_exports, &mut glob_exports);
    }
    resolve_module_exports(&mut module_exports, &glob_exports);

    let mut classes = HashSet::new();
    for (module, file) in &files {
        let mut visitor = ComponentVisitor::new(&module_exports, module.clone());
        visitor.visit_file(file);
        classes.extend(visitor.classes);
    }
    Ok(classes)
}

pub fn write_safelist(path: impl AsRef<Path>, classes: &HashSet<String>) -> io::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut sorted: Vec<&str> = classes.iter().map(String::as_str).collect();
    sorted.sort_unstable();
    fs::write(path, sorted.join("\n"))
}

fn walk_rs_files(dir: &Path, cb: &mut impl FnMut(&Path) -> io::Result<()>) -> io::Result<()> {
    let types = TypesBuilder::new()
        .add_defaults()
        .select("rust")
        .build()
        .map_err(io::Error::other)?;

    for result in WalkBuilder::new(dir).types(types).build() {
        match result {
            Ok(entry) if entry.file_type().is_some_and(|ft| ft.is_file()) => {
                cb(entry.path())?;
            }
            Ok(_) => {}
            Err(err) => println!("cargo:warning=granola-scanner: {err}"),
        }
    }
    Ok(())
}

fn module_path(root: &Path, path: &Path) -> Option<Vec<String>> {
    let relative = path.strip_prefix(root).ok()?;
    let mut module: Vec<String> = relative
        .parent()
        .into_iter()
        .flat_map(|parent| parent.components())
        .filter_map(|component| component.as_os_str().to_str().map(str::to_owned))
        .collect();
    if module.first().is_some_and(|component| component == "src") {
        module.remove(0);
    }
    let stem = relative.file_stem()?.to_str()?;
    if !matches!(stem, "lib" | "main" | "mod") {
        module.push(stem.to_owned());
    }
    Some(module)
}

fn split_relative_path<'a>(
    module: &[String],
    segments: &'a [String],
) -> Option<(Vec<String>, &'a [String])> {
    let (first, mut remaining) = segments.split_first()?;
    let module = match first.as_str() {
        "crate" => Vec::new(),
        "self" => module.to_vec(),
        "super" => {
            let mut module = module.to_vec();
            while remaining.first().is_some_and(|segment| segment == "super") {
                module.pop()?;
                remaining = &remaining[1..];
            }
            module.pop()?;
            module
        }
        _ => return None,
    };

    Some((module, remaining))
}

fn path_segments(path: &syn::Path) -> Vec<String> {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect()
}

fn collect_public_use_tree(
    tree: &UseTree,
    prefix: &[String],
    exports: &mut HashMap<Vec<String>, Vec<String>>,
    module: &[String],
    glob_exports: &mut Vec<GlobExport>,
) {
    match tree {
        UseTree::Path(path) => {
            let mut prefix = prefix.to_vec();
            prefix.push(path.ident.to_string());
            collect_public_use_tree(&path.tree, &prefix, exports, module, glob_exports);
        }
        UseTree::Name(name) => {
            let mut path = prefix.to_vec();
            path.push(name.ident.to_string());
            let mut local = module.to_vec();
            local.push(name.ident.to_string());
            exports.insert(local, path);
        }
        UseTree::Rename(rename) => {
            let path = if rename.ident == "self" {
                prefix.to_vec()
            } else {
                let mut path = prefix.to_vec();
                path.push(rename.ident.to_string());
                path
            };
            let mut local = module.to_vec();
            local.push(rename.rename.to_string());
            exports.insert(local, path);
        }
        UseTree::Group(group) => {
            for tree in &group.items {
                collect_public_use_tree(tree, prefix, exports, module, glob_exports);
            }
        }
        UseTree::Glob(_) => glob_exports.push(GlobExport {
            module: module.to_vec(),
            source: prefix.to_vec(),
        }),
    }
}

fn resolve_relative_module_path(module: &[String], path: &[String]) -> Option<Vec<String>> {
    let first = path.first()?;
    if first == GRANOLA {
        return None;
    }
    match first.as_str() {
        "crate" | "self" | "super" => {
            let (mut resolved, remaining) = split_relative_path(module, path)?;
            resolved.extend_from_slice(remaining);
            Some(resolved)
        }
        _ => Some(path.to_vec()),
    }
}

fn resolve_export_path(
    module: &[String],
    path: &[String],
    exports: &HashMap<Vec<String>, Vec<String>>,
) -> Option<Vec<String>> {
    if path.first().is_some_and(|first| first == GRANOLA) {
        return Some(path.to_vec());
    }
    let local = resolve_relative_module_path(module, path)?;
    exports.get(&local).cloned()
}

fn resolve_export_module(module: &[String], path: &[String]) -> Option<Vec<String>> {
    resolve_relative_module_path(module, path)
}

fn resolve_module_exports(
    exports: &mut HashMap<Vec<String>, Vec<String>>,
    glob_exports: &[GlobExport],
) {
    loop {
        let mut changed = false;
        let current: Vec<_> = exports
            .iter()
            .map(|(local, path)| (local.clone(), path.clone()))
            .collect();

        for (local, path) in current {
            let Some(module) = local.get(..local.len() - 1) else {
                continue;
            };
            let Some(resolved) = resolve_export_path(module, &path, exports) else {
                continue;
            };
            if resolved != path {
                exports.insert(local, resolved);
                changed = true;
            }
        }

        let current: Vec<_> = exports
            .iter()
            .map(|(local, path)| (local.clone(), path.clone()))
            .collect();
        for glob in glob_exports {
            let Some(source) = resolve_export_module(&glob.module, &glob.source) else {
                continue;
            };
            for (local, path) in &current {
                let Some(suffix) = local.strip_prefix(&source[..]) else {
                    continue;
                };
                let mut reexport = glob.module.clone();
                reexport.extend_from_slice(suffix);
                if let Entry::Vacant(e) = exports.entry(reexport) {
                    e.insert(path.clone());
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }
}

#[derive(Clone)]
struct GlobExport {
    module: Vec<String>,
    source: Vec<String>,
}

fn collect_module_exports(
    items: &[Item],
    module: &[String],
    exports: &mut HashMap<Vec<String>, Vec<String>>,
    glob_exports: &mut Vec<GlobExport>,
) {
    for item in items {
        match item {
            Item::Use(item) if !matches!(item.vis, syn::Visibility::Inherited) => {
                collect_public_use_tree(&item.tree, &[], exports, module, glob_exports);
            }
            Item::Mod(item) => {
                if let Some((_, items)) = &item.content {
                    let mut nested = module.to_vec();
                    nested.push(item.ident.to_string());
                    collect_module_exports(items, &nested, exports, glob_exports);
                }
            }
            _ => {}
        }
    }
}

#[derive(Default)]
struct Scope {
    module: Vec<String>,
    bindings: HashMap<String, Vec<String>>,
    daisyui_glob: bool,
    daisyui_component_globs: Vec<String>,
    macros_glob: bool,
}

struct ComponentVisitor<'exports> {
    scopes: Vec<Scope>,
    module: Vec<String>,
    module_exports: &'exports HashMap<Vec<String>, Vec<String>>,
    classes: HashSet<String>,
}

impl<'exports> ComponentVisitor<'exports> {
    fn new(
        module_exports: &'exports HashMap<Vec<String>, Vec<String>>,
        module: Vec<String>,
    ) -> Self {
        Self {
            scopes: Vec::new(),
            module,
            module_exports,
            classes: HashSet::new(),
        }
    }

    fn enter_scope<'a>(&mut self, uses: impl Iterator<Item = &'a syn::ItemUse>) {
        let uses: Vec<_> = uses.map(|item| &item.tree).collect();
        let mut scope = Scope {
            module: self.module.clone(),
            ..Default::default()
        };
        loop {
            let bindings = scope.bindings.clone();
            for tree in &uses {
                self.collect_use_tree(tree, &[], &mut scope);
            }
            if scope.bindings == bindings {
                break;
            }
        }
        self.scopes.push(scope);
    }

    fn leave_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_relative_segments(
        &self,
        segments: &[String],
        current_scope: Option<&Scope>,
    ) -> Option<Vec<String>> {
        let (module, remaining) = split_relative_path(&self.module, segments)?;

        if let Some((first, rest)) = remaining.split_first()
            && let Some(mut resolved) = current_scope
                .into_iter()
                .chain(self.scopes.iter().rev())
                .filter(|scope| scope.module == module)
                .find_map(|scope| scope.bindings.get(first).cloned())
        {
            resolved.extend_from_slice(rest);
            return Some(resolved);
        }

        let mut local = module;
        local.extend_from_slice(remaining);
        Some(self.module_exports.get(&local).cloned().unwrap_or(local))
    }

    fn resolve_segments(&self, segments: &[String]) -> Option<Vec<String>> {
        let first = segments.first()?;
        if first == GRANOLA {
            return Some(segments.to_vec());
        }
        if matches!(first.as_str(), "crate" | "self" | "super") {
            return self.resolve_relative_segments(segments, None);
        }

        self.scopes
            .iter()
            .rev()
            .find_map(|scope| {
                if let Some(path) = scope.bindings.get(first) {
                    let mut resolved = path.clone();
                    resolved.extend_from_slice(&segments[1..]);
                    Some(resolved)
                } else if segments.len() == 1 && scope.daisyui_glob {
                    Some(vec![GRANOLA.into(), DAISYUI.into(), first.clone()])
                } else if segments.len() == 1 && scope.macros_glob {
                    Some(vec![GRANOLA.into(), first.clone()])
                } else {
                    None
                }
            })
            .or_else(|| self.module_exports.get(segments).cloned())
    }

    fn resolve_import_segments(&self, segments: &[String], scope: &Scope) -> Option<Vec<String>> {
        let first = segments.first()?;
        if first == GRANOLA {
            return Some(segments.to_vec());
        }
        if matches!(first.as_str(), "crate" | "self" | "super") {
            return self.resolve_relative_segments(segments, Some(scope));
        }
        if let Some(path) = scope.bindings.get(first) {
            let mut resolved = path.clone();
            resolved.extend_from_slice(&segments[1..]);
            return Some(resolved);
        }
        self.resolve_segments(segments)
    }

    fn collect_use_tree(&self, tree: &UseTree, prefix: &[String], scope: &mut Scope) {
        match tree {
            UseTree::Path(p) => {
                let mut new_prefix = prefix.to_vec();
                new_prefix.push(p.ident.to_string());
                self.collect_use_tree(&p.tree, &new_prefix, scope);
            }
            UseTree::Name(n) => {
                let mut full = prefix.to_vec();
                full.push(n.ident.to_string());
                let path = self.resolve_import_segments(&full, scope).unwrap_or(full);
                scope.bindings.insert(n.ident.to_string(), path);
            }
            UseTree::Rename(r) => {
                let full = if r.ident == "self" {
                    prefix.to_vec()
                } else {
                    let mut full = prefix.to_vec();
                    full.push(r.ident.to_string());
                    full
                };
                let path = self.resolve_import_segments(&full, scope).unwrap_or(full);
                scope.bindings.insert(r.rename.to_string(), path);
            }
            UseTree::Group(g) => {
                for tree in &g.items {
                    self.collect_use_tree(tree, prefix, scope);
                }
            }
            UseTree::Glob(_) => {
                if let Some(path) = self.resolve_import_segments(prefix, scope) {
                    match path.as_slice() {
                        [granola, daisyui] if granola == GRANOLA && daisyui == DAISYUI => {
                            scope.daisyui_glob = true;
                        }
                        [granola, daisyui, component]
                            if granola == GRANOLA && daisyui == DAISYUI =>
                        {
                            if !scope.daisyui_component_globs.contains(component) {
                                scope.daisyui_component_globs.push(component.clone());
                            }
                        }
                        [granola, macros] if granola == GRANOLA && macros == "macros" => {
                            scope.macros_glob = true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn resolved_macro_name(&self, segments: &[String]) -> Option<String> {
        if let [name] = segments
            && self.scopes.iter().rev().any(|scope| scope.macros_glob)
        {
            return Some(name.clone());
        }

        let resolved = self.resolve_segments(segments)?;
        match resolved.as_slice() {
            [granola, name] if granola == GRANOLA => Some(name.clone()),
            [granola, macros, name] if granola == GRANOLA && macros == "macros" => {
                Some(name.clone())
            }
            _ => None,
        }
    }

    fn insert_component_class(&mut self, segments: &[String]) {
        if let Some(resolved) = self.resolve_segments(segments)
            && let [granola, daisyui, trailing @ ..] = resolved.as_slice()
            && granola == GRANOLA
            && daisyui == DAISYUI
            && !trailing.is_empty()
        {
            let trailing_refs: Vec<&str> = trailing.iter().map(String::as_str).collect();
            if let Some(class) = class_for_component_path(&trailing_refs) {
                self.classes.insert(class.to_owned());
                return;
            }
        }

        let seg_refs: Vec<&str> = segments.iter().map(String::as_str).collect();
        let class = self
            .scopes
            .iter()
            .rev()
            .flat_map(|scope| &scope.daisyui_component_globs)
            .find_map(|comp_name| {
                let comp = find_component(comp_name)?;
                comp.class_for_path(&seg_refs)
            });

        if let Some(class) = class {
            self.classes.insert(class.to_owned());
        }
    }

    fn insert_macro_class(&mut self, segments: &[String]) {
        if let Some(name) = self.resolved_macro_name(segments)
            && let Some(class) = class_for_macro(&name)
        {
            self.classes.insert(class.to_owned());
        }
    }

    fn visit_macro_arguments(&mut self, mac: &syn::Macro) {
        let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
        match parser.parse2(mac.tokens.clone()) {
            Ok(expressions) => {
                for expression in expressions {
                    self.visit_expr(&expression);
                }
            }
            Err(_) => self.visit_macro_tokens(mac.tokens.clone()),
        }
    }

    fn visit_macro_tokens(&mut self, tokens: TokenStream) {
        let tokens: Vec<_> = tokens.into_iter().collect();
        let mut index = 0;

        while index < tokens.len() {
            let TokenTree::Ident(ident) = &tokens[index] else {
                if let TokenTree::Group(group) = &tokens[index] {
                    self.visit_macro_tokens(group.stream());
                }
                index += 1;
                continue;
            };
            if index > 0
                && matches!(tokens[index - 1], TokenTree::Punct(ref punct) if punct.as_char() == '$')
            {
                index += 1;
                continue;
            }

            let mut segments = vec![ident.to_string()];
            let mut end = index + 1;
            while end + 2 < tokens.len()
                && matches!(tokens[end], TokenTree::Punct(ref punct) if punct.as_char() == ':')
                && matches!(tokens[end + 1], TokenTree::Punct(ref punct) if punct.as_char() == ':')
                && matches!(tokens[end + 2], TokenTree::Ident(_))
            {
                let TokenTree::Ident(next) = &tokens[end + 2] else {
                    unreachable!();
                };
                segments.push(next.to_string());
                end += 3;
            }

            if end + 1 < tokens.len()
                && matches!(tokens[end], TokenTree::Punct(ref punct) if punct.as_char() == '!')
                && let TokenTree::Group(group) = &tokens[end + 1]
            {
                self.insert_macro_class(&segments);
                self.visit_macro_tokens(group.stream());
                index = end + 2;
                continue;
            }

            self.insert_component_class(&segments);
            if let Some(TokenTree::Group(group)) = tokens.get(end) {
                self.visit_macro_tokens(group.stream());
                index = end + 1;
            } else {
                index = end;
            }
        }
    }
}

impl<'ast, 'exports> Visit<'ast> for ComponentVisitor<'exports> {
    fn visit_file(&mut self, file: &'ast syn::File) {
        self.enter_scope(file.items.iter().filter_map(|item| match item {
            Item::Use(item) => Some(item),
            _ => None,
        }));
        syn::visit::visit_file(self, file);
        self.leave_scope();
    }

    fn visit_block(&mut self, block: &'ast syn::Block) {
        self.enter_scope(block.stmts.iter().filter_map(|statement| match statement {
            Stmt::Item(Item::Use(item)) => Some(item),
            _ => None,
        }));
        syn::visit::visit_block(self, block);
        self.leave_scope();
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if let Some((_, items)) = &item.content {
            self.module.push(item.ident.to_string());
            self.enter_scope(items.iter().filter_map(|item| match item {
                Item::Use(item) => Some(item),
                _ => None,
            }));
            for item in items {
                self.visit_item(item);
            }
            self.leave_scope();
            self.module.pop();
        }
    }

    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        let _ = item;
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        self.insert_component_class(&path_segments(path));
        syn::visit::visit_path(self, path);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        self.insert_macro_class(&path_segments(&mac.path));
        self.visit_macro_arguments(mac);
    }

    fn visit_item_macro(&mut self, item: &'ast syn::ItemMacro) {
        if !item.mac.path.is_ident("macro_rules") {
            self.visit_macro(&item.mac);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn daisyui_safelist_has_no_duplicate_classes() {
        let classes = catalog::daisyui_safelist();
        let unique: std::collections::BTreeSet<_> = classes.iter().copied().collect();

        assert_eq!(classes.len(), unique.len());
    }

    #[test]
    fn component_catalog_resolves_modules_and_macros() {
        assert_eq!(class_for_component_path(&["Btn"]), Some("btn"));
        assert_eq!(class_for_component_path(&["btn", "Btn"]), Some("btn"));
        assert_eq!(
            class_for_component_path(&["btn", "Color", "Primary"]),
            Some("btn-primary")
        );
        assert_eq!(class_for_macro("btn"), Some("btn"));
        assert_eq!(class_for_macro("link"), Some("link"));

        assert_eq!(class_for_component_path(&["Card"]), Some("card"));
        assert_eq!(class_for_component_path(&["card", "Card"]), Some("card"));
        assert_eq!(
            class_for_component_path(&["card", "CardBody"]),
            Some("card-body")
        );
        assert_eq!(class_for_component_path(&["CardBody"]), Some("card-body"));
        assert_eq!(class_for_macro("card"), Some("card"));
        assert_eq!(class_for_macro("card_body"), Some("card-body"));
        assert_eq!(class_for_macro("card_title"), Some("card-title"));
        assert_eq!(class_for_macro("card_actions"), Some("card-actions"));
    }
}
