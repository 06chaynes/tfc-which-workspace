use crate::settings::Operators;
use crate::{error::FilterError, settings::Settings, WorkspaceVariablesOuter};

pub fn variable(
    workspace_variables: &mut WorkspaceVariablesOuter,
    config: &Settings,
) -> Result<(), FilterError> {
    // You have to return a bool from the closure.
    // If you return true, the element is not removed;
    // if you return false, it is removed.
    workspace_variables.workspaces.retain(|workspace| {
        let mut keep = false;
        for variable in config.clone().query.variables.unwrap() {
            match variable.operator {
                Operators::Equals => {
                    // Should be equal.
                    // so if it's not equal or not present we need to remove.
                    keep = false;
                    for var in workspace.variables.clone() {
                        if var.attributes.key == variable.key {
                            if let Some(value) = var.attributes.value {
                                if value == variable.value {
                                    keep = true;
                                }
                            }
                        }
                    }
                    if !keep {
                        return keep;
                    }
                }
                Operators::NotEquals => {
                    // Should not be equal.
                    // so if it's equal we need to remove.
                    keep = true;
                    for var in workspace.variables.clone() {
                        if var.attributes.key == variable.key {
                            if let Some(value) = var.attributes.value {
                                if value == variable.value {
                                    keep = false;
                                }
                            }
                        }
                    }
                    if !keep {
                        return keep;
                    }
                }
                Operators::Contains => {
                    // Should contain the query value.
                    // so if it doesn't contain the value or isn't present we need to remove.
                    keep = false;
                    for var in workspace.variables.clone() {
                        if var.attributes.key == variable.key {
                            if let Some(value) = var.attributes.value {
                                if value.contains(&variable.value.clone()) {
                                    keep = true;
                                }
                            }
                        }
                    }
                    if !keep {
                        return keep;
                    }
                }
                Operators::NotContains => {
                    // Should be a regex hit.
                    // so if it's not a hit or not present we need to remove.
                    keep = true;
                    for var in workspace.variables.clone() {
                        if var.attributes.key == variable.key {
                            if let Some(value) = var.attributes.value {
                                if value.contains(&variable.value.clone()) {
                                    keep = false;
                                }
                            }
                        }
                    }
                    if !keep {
                        return keep;
                    }
                }
            }
        }
        keep
    });
    Ok(())
}
