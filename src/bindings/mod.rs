// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use anyhow::Result;
use heck::ToKebabCase;
use serde::Deserialize;
use uniffi_bindgen::{BindingGenerator, GenerationSettings};

mod filters;
mod generator;
pub mod utils;

use crate::{
    bindings::generator::{Bindings, generate_node_bindings},
    utils::write_with_dirs,
};

pub struct NodeBindingGenerator {
    out_disable_auto_loading_lib: bool,
    out_import_extension: utils::ImportExtension,
}

impl NodeBindingGenerator {
    pub fn new(
        out_disable_auto_loading_lib: bool,
        out_import_extension: utils::ImportExtension,
    ) -> Self {
        Self {
            out_disable_auto_loading_lib,
            out_import_extension,
        }
    }
}

#[derive(Default, Deserialize)]
pub struct NodeBindingGeneratorConfig {
    // TODO: Add Node-specific configuration options.
}

impl BindingGenerator for NodeBindingGenerator {
    type Config = NodeBindingGeneratorConfig;

    fn new_config(&self, root_toml: &toml::Value) -> Result<Self::Config> {
        Ok(
            match root_toml.get("bindings").and_then(|b| b.get("node")) {
                Some(v) => v.clone().try_into()?,
                None => Default::default(),
            },
        )
    }

    fn update_component_configs(
        &self,
        _settings: &GenerationSettings,
        _components: &mut Vec<uniffi_bindgen::Component<Self::Config>>,
    ) -> Result<()> {
        Ok(())
    }

    fn write_bindings(
        &self,
        settings: &GenerationSettings,
        components: &[uniffi_bindgen::Component<Self::Config>],
    ) -> Result<()> {
        for uniffi_bindgen::Component { ci, config: _, .. } in components {
            let sys_ts_main_file_name = format!("{}-sys", ci.namespace().to_kebab_case());
            let node_ts_main_file_name = format!("{}-node", ci.namespace().to_kebab_case());

            let Bindings {
                node_ts_file_contents,
                index_ts_file_contents,
            } = generate_node_bindings(
                ci,
                sys_ts_main_file_name.as_str(),
                node_ts_main_file_name.as_str(),
                self.out_disable_auto_loading_lib,
                self.out_import_extension.clone(),
            )?;

            let node_ts_file_path = settings
                .out_dir
                .join(format!("{node_ts_main_file_name}.ts"));
            write_with_dirs(&node_ts_file_path, node_ts_file_contents)?;

            let index_template_path = settings.out_dir.join("index.ts");
            write_with_dirs(&index_template_path, index_ts_file_contents)?;
        }

        Ok(())
    }
}
