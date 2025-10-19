use crate::{Config, FileInfo, OutputFormat, Result, SortBy, TreeDisplay, TreeStats};
use ignore::WalkBuilder;
use serde_json::json;
use std::collections::HashMap;
use std::fs::{write};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

#[cfg(feature = "git")]
use crate::git::GitInfo;

pub struct TreeVisualizer {
    config: Config,
    display: TreeDisplay,
    #[cfg(feature = "git")]
    git_info: Option<GitInfo>,
    stats: TreeStats,
}

#[derive(Debug)]
struct TreeNode {
    info: FileInfo,
    children: Vec<TreeNode>,
}
impl TreeVisualizer {
    pub fn new(config: Config) -> Self {
        #[cfg(feature = "git")]
        let git_info = if config.git_ignore || config.git_status {
            Some(GitInfo::new(Path::new(".")))
        } else {
            None
        };

        #[cfg(feature = "git")]
        let display = if let Some(ref git_info) = git_info {
            TreeDisplay::new(config.clone()).with_git_info(git_info.clone())
        } else {
            TreeDisplay::new(config.clone())
        };

        #[cfg(not(feature = "git"))]
        let display = TreeDisplay::new(config.clone());

        Self {
            config,
            display,
            #[cfg(feature = "git")]
            git_info,
            stats: TreeStats::new(),
        }
    }

    pub fn visualize(&mut self, root_path: &Path) -> Result<()> {
        if !root_path.exists() {
            return Err(format!("Path '{}' does not exist", root_path.display()).into());
        }
        
        match self.config.output_format {
            OutputFormat::Tree => self.visualize_tree(root_path),
            OutputFormat::Json => self.visualize_json(root_path),
            OutputFormat::Csv => self.visualize_csv(root_path),
            OutputFormat::Markdown => self.visualize_markdown(root_path),
            OutputFormat::Xml => self.visualize_xml(root_path),
        }
    }

    pub fn visualize_to_file(&mut self, root_path: &Path, output_path: &Path) -> Result<()> {
        if !root_path.exists() {
            return Err(format!("Path '{}' does not exist", root_path.display()).into());
        }

        let output = self.generate_output(root_path)?;
        write(output_path, output)?;
        println!("Output written to {}", output_path.display());
        Ok(())
    }

    fn generate_output(&mut self, root_path: &Path) -> Result<String> {
        let mut output = String::new();

        match self.config.output_format {
            OutputFormat::Tree => {
                output.push_str(&self.display.format_path(root_path)?);
                output.push('\n');

                let entries= self.collect_entries(root_path)?;
                let tree = self.build_tree(entries, root_path)?;

                let filtered_tree = if self.config.filter_extension.is_some() {
                    self.filter_empty_directories(tree)
                } else {
                    tree
                };

                self.append_tree_nodes(&mut output, &filtered_tree,"", true)?;

                if self.config.show_stats {
                    self.stats.finalize();
                    output.push_str(&self.stats.display());
                }
            }
            OutputFormat::Json => {
                output = self.generate_json_output(root_path)?;
            }
            OutputFormat::Csv => {
                output = self.generate_csv_output(root_path)?;
            }
            OutputFormat::Markdown => {
                output = self.generate_markdown_output(root_path)?;
            }
            OutputFormat::Xml => {
                output = self.generate_xml_output(root_path)?;
            }
        }
        Ok(output)
        
    }

    fn visualize_tree(&mut self, root_path: &Path) -> Result<()> {
        println!("{}", self.display.format_path(root_path)?);

        let entries = self.collect_entries(root_path)?;
        let tree = self.build_tree(entries, root_path)?;

        let filtered_tree = if self.config.filter_extension.is_some() {
            self.filter_empty_directories(tree)
        } else {
            tree
        };

        self.print_tree_nodes(&filtered_tree, "", true)?;

        #[cfg(feature = "git")]
        if self.config.show_stats {
            self.stats.finalize();
            println!("{}", self.stats.display());
        }
        Ok(())
    }

    fn visualize_json(&mut self, root_path: &Path) -> Result<()> {
        let output = self.generate_json_output(root_path)?;
        println!("{}", output);
        Ok(())
    }

    fn visualize_csv(&mut self, root_path: &Path) -> Result<()> {
        let output = self.generate_csv_output(root_path)?;
        println!("{}", output);
        Ok(())
    }

    fn visualize_markdown(&mut self, root_path: &Path) -> Result<()> {
        let output = self.generate_markdown_output(root_path)?;
        println!("{}", output);
        Ok(())
    }

    fn visualize_xml(&mut self, root_path: &Path) -> Result<()> {
        let output = self.generate_xml_output(root_path)?;
        println!("{}", output);
        Ok(())
    }

    pub fn collect_entries(&mut self, root_path: &Path) -> Result<Vec<FileInfo>> {
        let mut entries = Vec::new();
        let mut file_count = 0;

        if self.config.git_ignore {
            // Use ignore crate for .gitignore support
            let walker = WalkBuilder::new(root_path)
                .max_depth(self.config.max_depth)
                .hidden(!self.config.show_hidden)
                .follow_links(self.config.follow_links)
                .build();

            for result in walker {
                let entry= result?;
                let path = entry.path();

                if path == root_path {
                    continue;
                }

                if !self.should_include_path(path)? {
                    continue;
                }

                if let Some(limit) = self.config.limit {
                    if file_count >= limit {
                        break;
                    }
                }
                
                let mut file_info = FileInfo::from_path(path)?;

                // Calculate checksum if requested
                if self.config.show_checksum && !file_info.is_dir {
                    file_info.calculate_checksum();
                }

                #[cfg(feature = "git")]
                self.stats.add_file(&file_info);

                entries.push(file_info);
                file_count +=1;
            }
        } else {
            // use walkdir for standard traversal
            let walker = WalkDir::new(root_path)
                .min_depth(1)
                .max_depth(self.config.max_depth.unwrap_or(usize::MAX))
                .follow_links(self.config.follow_links)
                .sort_by_file_name();

            for entry in walker {
                let entry = entry?;

                if !self.should_include_entry(&entry)? {
                    continue;
                }

                if let Some(limit) = self.config.limit {
                    if file_count >= limit {
                        break;
                    }
                }

                let mut file_info = FileInfo::from_path(entry.path())?;

                // Calculate checksum if requested
                if self.config.show_checksum && !file_info.is_dir {
                    file_info.calculate_checksum();
                }

                #[cfg(feature = "git")]
                self.stats.add_file(&file_info);

                entries.push(file_info);
                file_count +=1;
            }
        }
        self.sort_entries(&mut entries);
        Ok(entries)
    }

    fn should_include_entry(&self, entry: &DirEntry) -> Result<bool> {
        let path = entry.path();
        self.should_include_path(path)
    }

    fn should_include_path(&self, path:&Path) -> Result<bool> {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // skip hidden files unless requested
        if !self.config.show_hidden && file_name.starts_with('.') {
            return Ok(false);
        }

        // check gitignore
        #[cfg(feature = "git")]
        if let Some(ref git_info) = self.git_info {
            if self.config.git_ignore && git_info.is_ignored(path) {
                return Ok(false);
            }
        }

        //Check exclude patterns
        for pattern in &self.config.exclude_patterns {
            if crate::utils::matches_pattern(path, pattern) {
                return Ok(false);
            }
        }

        // Check include patterns
        if let Some(ref pattern) = self.config.include_patterns {
            if !crate::utils::matches_pattern(path, pattern) {
                return Ok(false);
            }
        }

        let metadata = std::fs::metadata(path).ok();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);

        // always include directories, they might contain files we want to show
        if is_dir {
            return Ok(true);
        }

        // show only directories if requested
        if self.config.directories_only {
            return Ok(false);
        }

        // show only files if requested
        if self.config.files_only && is_dir {
            return Ok(false);
        }

        // check size filters
        if let Some(ref metadata) = metadata {
            if !self.config.matches_size_filter(metadata.len()) {
                return Ok(false);
            }
        }

        // filter by extension if specified, only for files not directories
        if let Some(ref _ext_list) = self.config.filter_extension {
            let extensions = self.config.get_extension();
            if let Some(file_ext) = path.extension().and_then(|e| e.to_str()) {
                if !extensions.contains(&file_ext.to_lowercase()) {
                    return Ok(false);
                }
            } else if !extensions.is_empty() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn sort_entries(&self, entries: &mut Vec<FileInfo>) {
        entries.sort_by(|a, b| {
            let ordering = match self.config.sort_by {
                SortBy::Name => a.name.cmp(&b.name),
                SortBy::Size => a.size.cmp(&b.size),
                SortBy::Time => match(a.modified_time, b.modified_time) {
                    (Some(a_time), Some(b_time)) => a_time.cmp(&b_time),
                    (Some(_), None) => std::cmp::Ordering::Greater,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    (None, None) => std::cmp::Ordering::Equal,
                },
                SortBy::Extension => {
                    let a_ext = a.path.extension().and_then(|s| s.to_str()).unwrap_or("");
                    let b_ext = b.path.extension().and_then(|s| s.to_str()).unwrap_or("");
                    a_ext.cmp(b_ext)
                }
                SortBy::Type => {
                    // Directories first, then files
                    match (a.is_dir, b.is_dir) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.name.cmp(&b.name),
                    }
                }
            };
            if self.config.reverse_sort {
                ordering.reverse()
             } else {
                ordering
             }
        });
    }

    fn build_tree(&self, entries: Vec<FileInfo>, root_path: &Path) -> Result<Vec<TreeNode>> {
        let mut nodes = Vec::new();
        let mut entries_by_parent: HashMap<PathBuf, Vec<FileInfo>> = HashMap::new();

        // group entries by their parent directory
        for entry in entries {
            if let Some(parent) = entry.path.parent() {
                entries_by_parent
                    .entry(parent.to_path_buf())
                    .or_insert_with(Vec::new)
                    .push(entry);
            }
        }

        // get direct children of root
        if let Some(direct_children) =  entries_by_parent.get(root_path) {
            for child in direct_children {
                let node = self.build_node(child.clone(), &entries_by_parent)?;
                nodes.push(node);
            }
        }
        Ok(nodes)
    }

    fn build_node(&self, info: FileInfo, entries_by_parent: &HashMap<PathBuf, Vec<FileInfo>>) -> Result<TreeNode> {
        let mut children = Vec::new();

        if info.is_dir {
            if let Some(child_entries) = entries_by_parent.get(&info.path) {
                for child_info in child_entries {
                    let child_node = self.build_node(child_info.clone(), entries_by_parent)?;
                    children.push(child_node);
                }
            }
        }
        Ok(TreeNode {info, children})
    }

    fn filter_empty_directories(&self, nodes: Vec<TreeNode>) -> Vec<TreeNode> {
        nodes
            .into_iter()
            .filter_map(|mut node| {
                if node.info.is_dir {
                    //recursively filter children
                    node.children = self.filter_empty_directories(node.children);

                    // keep directory if it has children or if we are not filtering by extension
                    if !node.children.is_empty() || self.config.filter_extension.is_none() {
                        Some(node)
                    } else {
                        None
                    }
                } else {
                    // keep all files, they have already been filtered in should_include_entry
                    Some(node)
                }
            })
            .collect()
    }

    fn print_tree_nodes(&self, nodes: &[TreeNode], prefix: &str, _is_root: bool) -> Result<()> {
        for(i, node) in nodes.iter().enumerate() {
            let is_last = i == nodes.len() - 1;
            let connector = if is_last { "'-- " } else { "|-- "};
            let new_prefix = if is_last { "   " } else { "|   "};

            println!(
                "{}{}{}",
                prefix,
                connector,
                self.display.format_file_info(&node.info)?
            );

            if !node.children.is_empty() {
                let child_prefix = format!("{}{}", prefix, new_prefix);
                self.print_tree_nodes(&node.children, &child_prefix, false)?;
            }
        }
        Ok(())
    }

    fn append_tree_nodes(
        &self, output: &mut String, nodes: &[TreeNode], prefix: &str, _is_root: bool
    ) -> Result<()> {
        for(i, node) in nodes.iter().enumerate() {
            let is_last = i == nodes.len() - 1;
            let connector = if is_last { "'-- " } else { "-- " };
            let new_prefix = if is_last { "   " } else { "|   " };

            output.push_str(&format!(
                "{}{}{} \n",
                prefix,
                connector,
                self.display.format_file_info(&node.info)?
            ));

            if !node.children.is_empty() {
                let child_prefix = format!("{}{}", prefix, new_prefix);
                self.append_tree_nodes(output, &node.children, &child_prefix, false)?;
            }
        }
        Ok(())
    }

    fn generate_json_output(&mut self, root_path: &Path) -> Result<String> {
        let entries = self.collect_entries(root_path)?;
        let tree = self.build_tree(entries, root_path)?;

        let json_tree = self.tree_to_json(&tree)?;

        let output = json!({
            "root": root_path,
            "tree": json_tree,
            "stats": {
                "total_files": self.count_files(&tree),
                "total_dirs": self.count_dirs(&tree),
            }
        });

        Ok(serde_json::to_string_pretty(&output)?)
    }

    fn generate_csv_output(&mut self, root_path: &Path) -> Result<String> {
        let mut output = String::new();
        output.push_str("Name,Path,Size,IsDirectory,Modified,Permissions\n");

        let entries = self.collect_entries(root_path)?;
        for entry in entries {
            output.push_str(&self.display.format_file_info(&entry)?);
            output.push('\n');
        }
        Ok(output)
    }

    fn generate_markdown_output(&mut self, root_path: &Path) -> Result<String> {
        let mut output = String::new();
        output.push_str(&format!("Directory Tree: {}\n\n", root_path.display()));
        output.push_str("|  Name  |  Size |  Modified  |\n");
        output.push_str("|--------|-------|------------|\n");

        let entries = self.collect_entries(root_path)?;
        for entry in entries {
            output.push_str(&self.display.format_file_info(&entry)?);
            output.push('\n');
        }
        Ok(output)
    }

    fn generate_xml_output(&mut self, root_path: &Path)-> Result<String> {
        let mut output = String::new();
        output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        output.push_str(&format!("<tree root=\"{}\">\n", root_path.display()));

        let entries = self.collect_entries(root_path)?;
        for entry in entries {
            output.push_str("  ");
            output.push_str(&self.display.format_file_info(&entry)?);
            output.push('\n');
        }

        output.push_str("</tree>\n");
        Ok(output)
    }

    fn tree_to_json(&self, nodes: &[TreeNode]) -> Result<serde_json::Value> {
        let mut json_nodes = Vec::new();

        for node in nodes {
            let mut json_node = json!({
                "name": node.info.name,
                "path": node.info.path,
                "size": node.info.size,
                "is_dir": node.info.is_dir,
                "is_executable": node.info.is_executable,
                "is_symlink": node.info.is_symlink,
            });

            if !node.children.is_empty() {
                json_node["children"] = self.tree_to_json(&node.children)?;
            }
            json_nodes.push(json_node);
        }
        Ok(serde_json::Value::Array(json_nodes))
    }

    fn count_files(&self, nodes:&[TreeNode]) -> usize {
        let mut count = 0;
        for node in nodes {
            if !node.info.is_dir {
                count += 1;
            }
            count += self.count_files(&node.children);
        }
        count
    }

    fn count_dirs(&self, nodes: &[TreeNode]) -> usize {
        let mut count = 0;
        for node in nodes {
            if node.info.is_dir {
                count +=1;
            }
            count += self.count_dirs(&node.children);
        }
        count
    }
}