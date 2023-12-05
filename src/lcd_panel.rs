//! This code was produced from the work done by this person [this repo](https://github.com/anlumo/ESP32-8048S070-Rust/tree/main)
//! He did all the heavy lifting I just expanded on his work.
//! The default configuration and timings are based upon the Aliexpress Esp32S-8048S070C development board.
use std::ptr::null_mut;

use core::cell::UnsafeCell;

use esp_idf_sys::{
    esp,
    esp_lcd_new_rgb_panel,
    esp_lcd_panel_del,
    esp_lcd_panel_draw_bitmap,
    esp_lcd_panel_handle_t,
    esp_lcd_panel_init,
    esp_lcd_panel_reset,
    esp_lcd_rgb_panel_config_t,
    esp_lcd_rgb_panel_config_t__bindgen_ty_1,
    //esp_lcd_rgb_panel_get_frame_buffer,
    esp_lcd_rgb_timing_t,
    esp_lcd_rgb_timing_t__bindgen_ty_1,
    soc_periph_lcd_clk_src_t,
    soc_periph_lcd_clk_src_t_LCD_CLK_SRC_PLL160M,
    EspError,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PanelConfig {
    pub clk_src: soc_periph_lcd_clk_src_t,
    pub data_width: usize,
    pub bits_per_pixel: usize,
    pub num_fbs: usize,
    pub bounce_buffer_size_px: usize,
    pub sram_trans_align: usize,
    pub psram_trans_align: usize,
    pub hsync_gpio_num: i32,
    pub vsync_gpio_num: i32,
    pub de_gpio_num: i32,
    pub pclk_gpio_num: i32,
    pub disp_gpio_num: i32,
    pub data_gpio_nums: [i32; 16],
}

impl PanelConfig {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn clk_src(mut self, source: soc_periph_lcd_clk_src_t) -> Self {
        self.clk_src = source;
        self
    }

    #[must_use]
    pub fn data_width(mut self, data_width: usize) -> Self {
        self.data_width = data_width;
        self
    }

    #[must_use]
    pub fn bits_per_pixel(mut self, bits_per_pixel: usize) -> Self {
        self.bits_per_pixel = bits_per_pixel;
        self
    }

    #[must_use]
    pub fn num_fbs(mut self, num_fbs: usize) -> Self {
        self.num_fbs = num_fbs;
        self
    }

    #[must_use]
    pub fn bounce_buffer_size_px(mut self, bounce_buffer_size_px: usize) -> Self {
        self.bounce_buffer_size_px = bounce_buffer_size_px;
        self
    }

    #[must_use]
    pub fn sram_trans_align(mut self, sram_trans_align: usize) -> Self {
        self.sram_trans_align = sram_trans_align;
        self
    }

    #[must_use]
    pub fn hsync_gpio_num(mut self, hsync_gpio_num: i32) -> Self {
        self.hsync_gpio_num = hsync_gpio_num;
        self
    }

    #[must_use]
    pub fn vsync_gpio_num(mut self, vsync_gpio_num: i32) -> Self {
        self.vsync_gpio_num = vsync_gpio_num;
        self
    }

    #[must_use]
    pub fn de_gpio_num(mut self, de_gpio_num: i32) -> Self {
        self.de_gpio_num = de_gpio_num;
        self
    }

    #[must_use]
    pub fn pclk_gpio_num(mut self, pclk_gpio_num: i32) -> Self {
        self.pclk_gpio_num = pclk_gpio_num;
        self
    }

    #[must_use]
    pub fn disp_gpio_num(mut self, disp_gpio_num: i32) -> Self {
        self.disp_gpio_num = disp_gpio_num;
        self
    }

    #[must_use]
    pub fn data_gpio_nums(mut self, data_gpio_nums: [i32; 16]) -> Self {
        self.data_gpio_nums = data_gpio_nums;
        self
    }
}

impl Default for PanelConfig {
    fn default() -> Self {
        Self {
            clk_src: soc_periph_lcd_clk_src_t_LCD_CLK_SRC_PLL160M,
            data_width: 16,
            bits_per_pixel: 0,
            num_fbs: 1,
            bounce_buffer_size_px: 0,
            sram_trans_align: 8,
            psram_trans_align: 64,
            hsync_gpio_num: 39,
            vsync_gpio_num: 40,
            de_gpio_num: 41,
            pclk_gpio_num: 42,
            disp_gpio_num: -1,
            data_gpio_nums: [15, 7, 6, 5, 4, 9, 46, 3, 8, 16, 1, 14, 21, 47, 48, 45],
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PanelFlagsConfig {
    pub disp_active_low: u32,
    pub refresh_on_demand: u32,
    pub fb_in_psram: u32,
    pub double_fb: u32,
    pub no_fb: u32,
    pub bb_invalidate_cache: u32,
}

impl PanelFlagsConfig {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn disp_active_low(mut self, enable: bool) -> Self {
        self.disp_active_low = enable.into();
        self
    }

    #[must_use]
    pub fn refresh_on_demand(mut self, enable: bool) -> Self {
        self.refresh_on_demand = enable.into();
        self
    }

    #[must_use]
    pub fn fb_in_psram(mut self, enable: bool) -> Self {
        self.fb_in_psram = enable.into();
        self
    }

    #[must_use]
    pub fn double_fb(mut self, enable: bool) -> Self {
        self.double_fb = enable.into();
        self
    }

    #[must_use]
    pub fn no_fb(mut self, enable: bool) -> Self {
        self.no_fb = enable.into();
        self
    }

    #[must_use]
    pub fn bb_invalidate_cache(mut self, enable: bool) -> Self {
        self.bb_invalidate_cache = enable.into();
        self
    }
}

impl Default for PanelFlagsConfig {
    fn default() -> Self {
        Self {
            disp_active_low: 0,
            refresh_on_demand: 0,
            fb_in_psram: 1,
            double_fb: 0,
            no_fb: 0,
            bb_invalidate_cache: 0,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TimingsConfig {
    pub pclk_hz: u32,
    pub horz_res: u32,
    pub vert_res: u32,
    pub hsync_pulse_width: u32,
    pub hsync_back_porch: u32,
    pub hsync_front_porch: u32,
    pub vsync_pulse_width: u32,
    pub vsync_back_porch: u32,
    pub vsync_front_porch: u32,
}

impl TimingsConfig {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn pclk_hz(mut self, hz: u32) -> Self {
        self.pclk_hz = hz;
        self
    }

    #[must_use]
    pub fn horz_res(mut self, hres: u32) -> Self {
        self.horz_res = hres;
        self
    }

    #[must_use]
    pub fn vert_res(mut self, vres: u32) -> Self {
        self.vert_res = vres;
        self
    }

    #[must_use]
    pub fn hsync_pulse_width(mut self, hsync_pulse_width: u32) -> Self {
        self.hsync_pulse_width = hsync_pulse_width;
        self
    }

    #[must_use]
    pub fn hsync_back_porch(mut self, hsync_back_porch: u32) -> Self {
        self.hsync_back_porch = hsync_back_porch;
        self
    }

    #[must_use]
    pub fn hsync_front_porch(mut self, hsync_front_porch: u32) -> Self {
        self.hsync_front_porch = hsync_front_porch;
        self
    }

    #[must_use]
    pub fn vsync_pulse_width(mut self, vsync_pulse_width: u32) -> Self {
        self.vsync_pulse_width = vsync_pulse_width;
        self
    }

    #[must_use]
    pub fn vsync_back_porch(mut self, vsync_back_porch: u32) -> Self {
        self.vsync_back_porch = vsync_back_porch;
        self
    }

    #[must_use]
    pub fn vsync_front_porch(mut self, vsync_front_porch: u32) -> Self {
        self.vsync_front_porch = vsync_front_porch;
        self
    }
}

impl Default for TimingsConfig {
    fn default() -> Self {
        Self {
            pclk_hz: (16 * 1000 * 1000),
            horz_res: 800,
            vert_res: 480,
            hsync_pulse_width: 30,
            hsync_back_porch: 16,
            hsync_front_porch: 210,
            vsync_pulse_width: 13,
            vsync_back_porch: 10,
            vsync_front_porch: 22,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TimingFlagsConfig {
    pub hsync_idle_low: u32,
    pub vsync_idle_low: u32,
    pub de_idle_high: u32,
    pub pclk_active_neg: u32,
    pub pclk_idle_high: u32,
}

impl TimingFlagsConfig {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn hsync_idle_low(mut self, enable: bool) -> Self {
        self.hsync_idle_low = enable.into();
        self
    }

    #[must_use]
    pub fn vsync_idle_low(mut self, enable: bool) -> Self {
        self.vsync_idle_low = enable.into();
        self
    }

    #[must_use]
    pub fn de_idle_high(mut self, enable: bool) -> Self {
        self.de_idle_high = enable.into();
        self
    }

    #[must_use]
    pub fn pclk_active_neg(mut self, enable: bool) -> Self {
        self.pclk_active_neg = enable.into();
        self
    }

    #[must_use]
    pub fn set_pclk_idle_high(mut self, enable: bool) -> Self {
        self.pclk_idle_high = enable.into();
        self
    }
}

impl Default for TimingFlagsConfig {
    fn default() -> Self {
        Self {
            hsync_idle_low: 0,
            vsync_idle_low: 0,
            de_idle_high: 0,
            pclk_active_neg: 1,
            pclk_idle_high: 0,
        }
    }
}

pub struct LcdPanel {
    panel: esp_lcd_panel_handle_t,
}

impl LcdPanel {
    pub fn new(
        panel_config: &PanelConfig,
        panel_flags_config: &PanelFlagsConfig,
        timing_config: &TimingsConfig,
        timing_flags_config: &TimingFlagsConfig,
    ) -> Result<Self, EspError> {
        let timings = esp_lcd_rgb_timing_t {
            pclk_hz: timing_config.pclk_hz,
            h_res: timing_config.horz_res,
            v_res: timing_config.vert_res,
            hsync_pulse_width: timing_config.hsync_pulse_width,
            hsync_back_porch: timing_config.hsync_back_porch,
            hsync_front_porch: timing_config.hsync_front_porch,
            vsync_pulse_width: timing_config.vsync_pulse_width,
            vsync_back_porch: timing_config.vsync_back_porch,
            vsync_front_porch: timing_config.vsync_front_porch,
            flags: {
                let mut flags = esp_lcd_rgb_timing_t__bindgen_ty_1::default();
                flags.set_vsync_idle_low(timing_flags_config.vsync_idle_low);
                flags.set_vsync_idle_low(timing_flags_config.vsync_idle_low);
                flags.set_de_idle_high(timing_flags_config.de_idle_high);
                flags.set_pclk_active_neg(timing_flags_config.pclk_active_neg);
                flags.set_pclk_idle_high(timing_flags_config.pclk_idle_high);
                flags
            },
        };

        let panel_config = esp_lcd_rgb_panel_config_t {
            clk_src: panel_config.clk_src,
            timings,
            data_width: panel_config.data_width,
            bits_per_pixel: panel_config.bits_per_pixel,
            num_fbs: panel_config.num_fbs,
            bounce_buffer_size_px: panel_config.bounce_buffer_size_px,
            sram_trans_align: panel_config.sram_trans_align,
            psram_trans_align: panel_config.psram_trans_align,
            hsync_gpio_num: panel_config.hsync_gpio_num,
            vsync_gpio_num: panel_config.vsync_gpio_num,
            de_gpio_num: panel_config.de_gpio_num,
            pclk_gpio_num: panel_config.pclk_gpio_num,
            disp_gpio_num: panel_config.disp_gpio_num,
            data_gpio_nums: panel_config.data_gpio_nums,
            flags: {
                let mut flags = esp_lcd_rgb_panel_config_t__bindgen_ty_1::default();
                flags.set_disp_active_low(panel_flags_config.disp_active_low);
                flags.set_refresh_on_demand(panel_flags_config.refresh_on_demand);
                flags.set_fb_in_psram(panel_flags_config.fb_in_psram);
                flags.set_double_fb(panel_flags_config.double_fb);
                flags.set_bb_invalidate_cache(panel_flags_config.bb_invalidate_cache);
                flags
            },
        };

        let mut panel = null_mut() as esp_lcd_panel_handle_t;

        unsafe {
            // create panel
            esp!(esp_lcd_new_rgb_panel(&panel_config, &mut panel))?;

            // reset panel
            esp!(esp_lcd_panel_reset(panel))?;

            // initialize panel
            esp!(esp_lcd_panel_init(panel))?;
        };

        Ok(Self { panel })
    }

    ///
    /// Sets pixel colors in a rectangular region.
    ///
    /// The color values from the `colors` iterator will be drawn to the given region starting
    /// at the top left corner and continuing, row first, to the bottom right corner. No bounds
    /// checking is performed on the `colors` iterator and drawing will wrap around if the
    /// iterator returns more color values than the number of pixels in the given region.
    ///
    /// # Arguments
    ///
    /// * `sx` - x coordinate start
    /// * `sy` - y coordinate start
    /// * `ex` - x coordinate end
    /// * `ey` - y coordinate end
    /// * `colors` - anything that can provide `IntoIterator<Item = lvgl::Color>` to iterate over pixel data
    pub fn set_pixels_lvgl_color<T>(
        &mut self,
        sx: i32,
        sy: i32,
        ex: i32,
        ey: i32,
        colors: T,
    ) -> Result<(), EspError>
    where
        T: IntoIterator<Item = lvgl::Color>,
    {
        let iter = UnsafeCell::new(colors);
        unsafe {
            esp!(esp_lcd_panel_draw_bitmap(
                self.panel,
                sx,
                sy,
                ex,
                ey,
                &iter as *const _ as _, //colors.as_ptr() as *const c_void,
            ))?;
        };

        Ok(())
    }
}

impl Drop for LcdPanel {
    fn drop(&mut self) {
        esp!(unsafe { esp_lcd_panel_del(self.panel) }).unwrap();
    }
}
