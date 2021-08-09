use std::collections::HashMap;

use anyhow::Result;
use wgpu::util::DeviceExt;

use bytemuck::{Pod, Zeroable};

// use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};

use nalgebra as na;
use nalgebra_glm as glm;

use crate::{geometry::Point, view::View};

pub mod egui_wgpu;

use egui_wgpu::*;

pub struct Gui {
    pub platform: Platform,
    pub egui_rpass: RenderPass,
    pub screen_descriptor: ScreenDescriptor,
}

impl Gui {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
        scale_factor: f64,
    ) -> Self {
        let screen_descriptor = ScreenDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor: 1.0,
        };

        let egui_rpass = RenderPass::new(&device, format, 1);

        let ctx = egui::CtxRef::default();

        let font_defs = {
            use egui::FontFamily as Family;
            use egui::TextStyle as Style;

            let mut font_defs = egui::FontDefinitions::default();
            let fam_size = &mut font_defs.family_and_size;

            fam_size.insert(Style::Small, (Family::Proportional, 12.0));
            fam_size.insert(Style::Body, (Family::Proportional, 16.0));
            fam_size.insert(Style::Button, (Family::Proportional, 18.0));
            fam_size.insert(Style::Heading, (Family::Proportional, 22.0));
            font_defs
        };

        let platform = Platform::new(PlatformDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor,
            font_definitions: font_defs,
            style: Default::default(),
        });

        Self {
            platform,
            egui_rpass,
            screen_descriptor,
        }
    }

    pub fn draw_chr_labels(&self, offsets: &HashMap<String, usize>, view: View, y: f32, log: bool) {
        let ctx = self.platform.context();
        let painter = ctx.layer_painter(painter_layer());

        let screen_rect = ctx.input().screen_rect();

        for (chr, offset) in offsets {
            let offset = *offset;

            let matrix = view.to_scaled_matrix();
            let screen_pos = matrix * glm::vec4(offset as f32, 0.0, 0.0, 1.0);

            let x = screen_pos[0];
            let x_ = (x + 1.0) * (screen_rect.width() / 2.0);

            let screen_2d = Point::new(x_, y);

            let rect = painter.text(
                screen_2d.into(),
                egui::Align2::CENTER_CENTER,
                chr,
                egui::TextStyle::Heading,
                egui::Color32::RED,
            );

            if log {
                let string = format!("x: {}, y: {}", rect.center().x, rect.center().y);
                web_sys::console::log_1(&string.into());
            }
        }
    }
}

fn painter_layer() -> egui::LayerId {
    egui::LayerId::new(
        egui::Order::Background,
        egui::Id::new("gui_text_background"),
    )
}
