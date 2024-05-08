use std::collections::HashMap;

use crate::consts::{DIRECTION_ATTRIBUTE_KEY, DISPLAY_ATTRIBUTE_KEY, EACH_ATTRIBUTE_KEY, EXPAND_ATTRIBUTE_KEY, FONT_SIZE_ATTRIBUTE_KEY, GAP_ATTRIBUTE_KEY, MARGIN_ATTRIBUTE_KEY, PADDING_ATTRIBUTE_KEY, SCROLLABLE_ATTRIBUTE_KEY};

use super::{container_direction::ContainerDirection, container_expand::ContainerExpand};

pub trait GetAttributes {
    fn direction(&self) -> ContainerDirection;
    fn expand(&self) -> Option<ContainerExpand>;
    fn gap(&self) -> u16;
    fn margin(&self) -> u16;
    fn padding(&self) -> u16;

    fn font_size(&self) -> Option<u8>;

    fn scrollable(&self) -> Option<ContainerDirection>;

    fn each(&self) -> String;
    fn display(&self) -> String;
}

impl GetAttributes for HashMap<String, Option<String>> {
    fn direction(&self) -> ContainerDirection {
        self.get(DIRECTION_ATTRIBUTE_KEY)
            .and_then(|v| v.as_ref())
            .cloned().into()
    }

    fn expand(&self) -> Option<ContainerExpand> {
        self.get(EXPAND_ATTRIBUTE_KEY)
            .map(|v| ContainerExpand::from(v.to_owned()))
    }

    fn gap(&self) -> u16 {
        self.get(GAP_ATTRIBUTE_KEY)
            .and_then(|v| v.as_ref())
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
    }

    fn margin(&self) -> u16 {
        self.get(MARGIN_ATTRIBUTE_KEY)
            .and_then(|v| v.as_ref())
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
    }

    fn padding(&self) -> u16 {
        self.get(PADDING_ATTRIBUTE_KEY)
            .and_then(|v| v.as_ref())
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
    }

    fn font_size(&self) -> Option<u8> {
        self.get(FONT_SIZE_ATTRIBUTE_KEY)
            .and_then(|v| v.as_ref())
            .and_then(|v| v.parse().ok())
    }

    fn scrollable(&self) -> Option<ContainerDirection> {
        self.get(SCROLLABLE_ATTRIBUTE_KEY)
            .map(|v| ContainerDirection::from(v.to_owned()))
    }

    fn each(&self) -> String {
        self.get(EACH_ATTRIBUTE_KEY)
            .and_then(|v| v.clone()).unwrap()
    }

    fn display(&self) -> String {
        self.get(DISPLAY_ATTRIBUTE_KEY)
            .and_then(|v| v.clone()).unwrap()
    }
}