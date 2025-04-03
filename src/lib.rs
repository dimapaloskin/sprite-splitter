use photon_rs::native::{open_image, save_image};
use photon_rs::{PhotonImage, transform};
use std::{ffi::OsStr, path::Path};

pub struct SpriteSheet<'a> {
    img: PhotonImage,
    stem: &'a OsStr,
    ext: &'a OsStr,
    output_dir: &'a Path,
    rows: u32,
    columns: u32,
    img_width: u32,
    img_height: u32,
    sprite_width: u32,
    sprite_height: u32,
    cur_row: u32,
    cur_col: u32,
}

impl<'a> SpriteSheet<'a> {
    pub fn new(file_path: &'a Path, output_dir: &'a Path, rows: u32, columns: u32) -> Self {
        let img = open_image(&file_path.to_str().unwrap()).unwrap();

        let w = img.get_width();
        let h = img.get_height();

        let sprite_width = w / columns;
        let sprite_height = h / rows;

        Self {
            img,
            stem: file_path.file_stem().unwrap(),
            ext: file_path.extension().unwrap(),
            output_dir,
            rows,
            columns,
            img_width: w,
            img_height: h,
            sprite_width,
            sprite_height,
            cur_row: 0,
            cur_col: 0,
        }
    }

    pub fn img_width(&self) -> u32 {
        self.img_width
    }

    pub fn img_height(&self) -> u32 {
        self.img_height
    }

    pub fn sprite_width(&self) -> u32 {
        self.sprite_width
    }

    pub fn sprite_height(&self) -> u32 {
        self.sprite_height
    }
}

impl<'a> Iterator for SpriteSheet<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.rows == 0 || self.columns == 0 {
            return None;
        }

        if self.cur_row >= self.rows {
            return None;
        }

        let x1 = self.cur_col * self.sprite_width;
        let y1 = self.cur_row * self.sprite_height;
        let x2 = x1 + self.sprite_width;
        let y2 = y1 + self.sprite_height;

        let cropped = transform::crop(&mut self.img, x1, y1, x2, y2);

        let output_path = self.output_dir.join(format!(
            "{}_{}_{}.{}",
            self.stem.to_str().unwrap(),
            self.cur_row + 1,
            self.cur_col + 1,
            self.ext.to_str().unwrap()
        ));

        save_image(cropped, &output_path.to_str().unwrap()).unwrap();

        self.cur_col += 1;
        if self.cur_col >= self.columns {
            self.cur_col = 0;
            self.cur_row += 1;
        }

        Some(())
    }
}
