use crate::{Click, Component, UpdateElement};
use crossterm::style::Color;
use crossterm::Result;
use uuid::Uuid;
use crate::screen::ClickAction;

#[derive(Debug, Clone)]
pub struct ButtonComponent {
    id: Uuid,
    label: Box<str>,
    width: usize,
    height: usize,
    changed: bool,
    pub click_action: ClickAction,
}

impl ButtonComponent {
    pub fn new(label: Box<str>, width: usize, height: usize, click_action: ClickAction) -> Self {
        return ButtonComponent {
            id: Uuid::new_v4(),
            label,
            width,
            height,
            changed: true,
            click_action,
        };
    }
    pub fn update_click_action(&mut self, click_action: ClickAction) {
        self.click_action = click_action;
    }
}

impl Component for ButtonComponent {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_size(&self) -> (usize, usize) {
        return (self.width, self.height);
    }

    fn get_updates(&mut self) -> Result<Vec<UpdateElement>> {
        let mut updates = vec![];
        if self.changed {
            let mut y = 0;
            if y > 1 {
                y = y / 2;
            }
            let mut x = 0;
            let label_len = self.label.len();
            if self.width > label_len {
                x = self.width / 2 - label_len / 2;
            }
            let i = 0;
            for c in self.label.chars() {
                updates.push(UpdateElement {
                    point: (x,y).into(),
                    value: c,
                    fg: None,
                });
                x += 1;
                if x >= self.width {
                    break;
                }
            }
        }
        return Ok(updates);
    }

    fn handle_click(&mut self, click: Click) -> Result<ClickAction> {
        Ok(match click {
            Click::Left(_) => self.click_action.clone(),
            _ => ClickAction::None
        })
    }
}
