use super::Point;

const TRACE: bool = false;

/// Modes for hinting.
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum HinterMode {
    /// "Full" hinting mode. May generate rough outlines and poor horizontal
    /// spacing.
    Legacy,
    /// Subpixel mode for grayscale rendering.
    GrayscaleSubpixel,
    /// Cleartype subpixel mode.
    Subpixel,
    /// Same as subpixel, but always prevents adjustment in the horizontal
    /// direction.
    Modern,
}

impl Default for HinterMode {
    fn default() -> Self {
        Self::Modern
    }
}

/// State for the TrueType hinter.
#[derive(Copy, Clone)]
pub struct HinterState {
    gs: GraphicsState,
    default_gs: GraphicsState,
    ppem: u16,
    point_size: i32,
    scale: i32,
    coord_count: u16,
    compat: bool,
    mode: HinterMode,
}

impl Default for HinterState {
    fn default() -> Self {
        Self {
            gs: DEFAULT_GRAPHICS_STATE,
            default_gs: DEFAULT_GRAPHICS_STATE,
            ppem: 0,
            point_size: 0,
            scale: 0,
            coord_count: 0,
            compat: false,
            mode: HinterMode::Modern,
        }
    }
}

impl HinterState {
    /// Returns true if hinting is enabled for this state.
    pub fn hinting_enabled(&self) -> bool {
        self.gs.instruct_control & 1 == 0
    }

    /// Returns true if compatibility mode is enabled for this state.
    pub fn compat_enabled(&self) -> bool {
        self.compat
    }
}

/// Function or instruction definition for TrueType hinting.
#[derive(Copy, Clone, Default)]
pub struct Function {
    program: u8,
    active: bool,
    offset: u32,
    end: u32,
    opcode: u16,
}

/// Glyph zone for TrueType hinting.
pub struct Zone<'a> {
    unscaled: &'a mut [Point],
    original: &'a mut [Point],
    points: &'a mut [Point],
    tags: &'a mut [u8],
    contours: &'a [u16],
}

impl<'a> Zone<'a> {
    /// Creates a new hinting zone.
    pub fn new(
        unscaled: &'a mut [Point],
        original: &'a mut [Point],
        points: &'a mut [Point],
        tags: &'a mut [u8],
        contours: &'a [u16],
    ) -> Self {
        Self {
            unscaled,
            original,
            points,
            tags,
            contours,
        }
    }

    fn shift(&mut self, is_x: bool, p1: usize, p2: usize, p: usize) -> Option<()> {
        if p1 > p2 || p1 > p || p > p2 {
            return Some(());
        }
        let z = self;
        if is_x {
            let delta = z.points.get(p)?.x - z.original.get(p)?.x;
            if delta != 0 {
                let (first, second) = z.points.get_mut(p1..=p2)?.split_at_mut(p - p1);
                for point in first.iter_mut().chain(second.get_mut(1..)?) {
                    point.x += delta;
                }
            }
        } else {
            let delta = z.points.get(p)?.y - z.original.get(p)?.y;
            if delta != 0 {
                let (first, second) = z.points.get_mut(p1..=p2)?.split_at_mut(p - p1);
                for point in first.iter_mut().chain(second.get_mut(1..)?) {
                    point.y += delta;
                }
            }
        }
        Some(())
    }

    fn interpolate(
        &mut self,
        is_x: bool,
        p1: usize,
        p2: usize,
        mut ref1: usize,
        mut ref2: usize,
    ) -> Option<()> {
        if p1 > p2 {
            return Some(());
        }
        let z = self;
        let max_points = z.points.len();
        if ref1 >= max_points || ref2 >= max_points {
            return Some(());
        }
        let (mut orus1, mut orus2) = if is_x {
            (z.unscaled.get(ref1)?.x, z.unscaled.get(ref2)?.x)
        } else {
            (z.unscaled.get(ref1)?.y, z.unscaled.get(ref2)?.y)
        };
        if orus1 > orus2 {
            core::mem::swap(&mut orus1, &mut orus2);
            core::mem::swap(&mut ref1, &mut ref2);
        }
        let (org1, org2, cur1, cur2) = if is_x {
            (
                z.original.get(ref1)?.x,
                z.original.get(ref2)?.x,
                z.points.get(ref1)?.x,
                z.points.get(ref2)?.x,
            )
        } else {
            (
                z.original.get(ref1)?.y,
                z.original.get(ref2)?.y,
                z.points.get(ref1)?.y,
                z.points.get(ref2)?.y,
            )
        };
        let delta1 = cur1 - org1;
        let delta2 = cur2 - org2;
        let iter = z
            .original
            .get(p1..=p2)?
            .iter()
            .zip(z.unscaled.get(p1..=p2)?)
            .zip(z.points.get_mut(p1..=p2)?);
        if cur1 == cur2 || orus1 == orus2 {
            if is_x {
                for ((orig, _unscaled), point) in iter {
                    let a = orig.x;
                    point.x = if a <= org1 {
                        a + delta1
                    } else if a >= org2 {
                        a + delta2
                    } else {
                        cur1
                    };
                }
            } else {
                for ((orig, _unscaled), point) in iter {
                    let a = orig.y;
                    point.y = if a <= org1 {
                        a + delta1
                    } else if a >= org2 {
                        a + delta2
                    } else {
                        cur1
                    };
                }
            }
        } else {
            let scale = div(cur2 - cur1, orus2 - orus1);
            if is_x {
                for ((orig, unscaled), point) in iter {
                    let a = orig.x;
                    point.x = if a <= org1 {
                        a + delta1
                    } else if a >= org2 {
                        a + delta2
                    } else {
                        cur1 + mul(unscaled.x - orus1, scale)
                    };
                }
            } else {
                for ((orig, unscaled), point) in iter {
                    let a = orig.y;
                    point.y = if a <= org1 {
                        a + delta1
                    } else if a >= org2 {
                        a + delta2
                    } else {
                        cur1 + mul(unscaled.y - orus1, scale)
                    };
                }
            }
        }
        Some(())
    }
}

/// TrueType hinting engine.
pub struct Hinter<'a> {
    store: &'a mut [i32],
    cvt: &'a mut [i32],
    fdefs: &'a mut [Function],
    idefs: &'a mut [Function],
    stack: &'a mut [i32],
    twilight: Zone<'a>,
    glyph: Zone<'a>,
    coords: &'a [i16],
    axis_count: u16,
    zp0: u8,
    zp1: u8,
    zp2: u8,
    ppem: u16,
    point_size: i32,
    scale: i32,
    yscale: i32,
    rotated: bool,
    round_state: u8,
    round_threshold: i32,
    round_phase: i32,
    round_period: i32,
    project_state: u8,
    dual_project_state: u8,
    move_state: u8,
    pv: Point,
    dv: Point,
    fv: Point,
    fdotp: i32,
    iupx: bool,
    iupy: bool,
    v35: bool,
    subpixel: bool,
    compat: bool,
}

impl<'a> Hinter<'a> {
    /// Creates a new hinting engine.
    pub fn new(
        storage: &'a mut [i32],
        cvt: &'a mut [i32],
        function_defs: &'a mut [Function],
        instruction_defs: &'a mut [Function],
        stack: &'a mut [i32],
        twilight: Zone<'a>,
        glyph: Zone<'a>,
        coords: &'a [i16],
        axis_count: u16,
    ) -> Self {
        Self {
            store: storage,
            cvt,
            fdefs: function_defs,
            idefs: instruction_defs,
            stack,
            twilight,
            glyph,
            coords,
            axis_count,
            zp0: 1,
            zp1: 1,
            zp2: 1,
            ppem: 0,
            point_size: 0,
            scale: 0,
            yscale: 0,
            rotated: false,
            iupx: false,
            iupy: false,
            dv: Point::new(0x4000, 0),
            fv: Point::new(0x4000, 0),
            round_phase: 0,
            round_period: 64,
            pv: Point::new(0x4000, 0),
            fdotp: 0x4000,
            round_threshold: 0,
            round_state: 1,
            project_state: 0,
            dual_project_state: 0,
            move_state: 0,
            v35: false,
            subpixel: true,
            compat: false,
        }
    }

    pub fn run_fpgm<'b>(&mut self, state: &'b mut HinterState, fpgm: &[u8]) -> bool {
        let programs = [fpgm, &[], &[]];
        for f in self.fdefs.iter_mut() {
            *f = Function::default();
        }
        for f in self.idefs.iter_mut() {
            *f = Function::default();
        }
        state.ppem = 0;
        state.point_size = 0;
        state.scale = 0;
        state.mode = HinterMode::Modern;
        state.gs = DEFAULT_GRAPHICS_STATE;
        state.default_gs = DEFAULT_GRAPHICS_STATE;
        state.coord_count = self.axis_count;
        self.execute(state, programs, 0, false).is_some()
    }

    pub fn run_prep<'b>(
        &mut self,
        state: &'b mut HinterState,
        mode: HinterMode,
        fpgm: &[u8],
        prep: &[u8],
        ppem: u16,
        scale: i32,
    ) -> bool {
        let programs = [fpgm, prep, &[]];
        for p in self.twilight.unscaled.iter_mut() {
            *p = Point::new(0, 0);
        }
        for p in self.twilight.original.iter_mut() {
            *p = Point::new(0, 0);
        }
        for p in self.twilight.points.iter_mut() {
            *p = Point::new(0, 0);
        }
        state.mode = mode;
        state.ppem = ppem;
        state.scale = scale;
        state.point_size = muldiv(ppem as i32, 64 * 72, 72);
        state.coord_count = self.axis_count;
        self.ppem = state.ppem;
        self.point_size = state.point_size;
        self.scale = state.scale;
        self.yscale = state.scale;
        let res = self.execute(state, programs, 1, false);
        if res.is_some() {
            state.default_gs = state.gs;
            true
        } else {
            false
        }
    }

    pub fn run<'b>(
        &mut self,
        state: &'b mut HinterState,
        fpgm: &[u8],
        prep: &[u8],
        ins: &[u8],
        is_composite: bool,
    ) -> bool {
        let programs = [fpgm, prep, ins];
        self.ppem = state.ppem;
        self.point_size = state.point_size;
        self.scale = state.scale;
        self.yscale = state.scale;
        if is_composite {
            self.yscale = 1 << 16;
        }
        if state.default_gs.instruct_control & 2 != 0 {
            state.gs = DEFAULT_GRAPHICS_STATE;
        } else {
            state.gs = state.default_gs;
        }
        let res = self.execute(state, programs, 2, is_composite);
        self.yscale = self.scale;
        res.is_some()
    }
}

impl<'a> Hinter<'a> {
    #[inline(always)]
    fn zp(&self, zone: u8) -> &Zone {
        if zone == 1 {
            return &self.glyph;
        }
        &self.twilight
    }

    #[inline(always)]
    fn zp_mut<'b>(&'b mut self, zone: u8) -> &'b mut Zone<'a> {
        if zone == 1 {
            return &mut self.glyph;
        }
        &mut self.twilight
    }

    #[inline(always)]
    fn zp0(&self) -> &Zone {
        if self.zp0 == 1 {
            return &self.glyph;
        }
        &self.twilight
    }

    #[inline(always)]
    fn zp0_mut<'b>(&'b mut self) -> &'b mut Zone<'a> {
        if self.zp0 == 1 {
            return &mut self.glyph;
        }
        &mut self.twilight
    }

    #[inline(always)]
    fn zp1(&self) -> &Zone {
        if self.zp1 == 1 {
            return &self.glyph;
        }
        &self.twilight
    }

    #[inline(always)]
    fn zp1_mut<'b>(&'b mut self) -> &'b mut Zone<'a> {
        if self.zp1 == 1 {
            return &mut self.glyph;
        }
        &mut self.twilight
    }

    #[inline(always)]
    fn zp2(&self) -> &Zone {
        if self.zp2 == 1 {
            return &self.glyph;
        }
        &self.twilight
    }

    #[inline(always)]
    fn zp2_mut<'b>(&'b mut self) -> &'b mut Zone<'a> {
        if self.zp2 == 1 {
            return &mut self.glyph;
        }
        &mut self.twilight
    }

    fn round(&self, distance: i32) -> i32 {
        match self.round_state {
            ROUND_TO_HALF_GRID => {
                if distance >= 0 {
                    (floor(distance) + 32).max(0)
                } else {
                    (-(floor(-distance) + 32)).min(0)
                }
            }
            ROUND_TO_GRID => {
                if distance >= 0 {
                    round(distance).max(0)
                } else {
                    (-round(-distance)).min(0)
                }
            }
            ROUND_TO_DOUBLE_GRID => {
                if distance >= 0 {
                    round_pad(distance, 32).max(0)
                } else {
                    (-round_pad(-distance, 32)).min(0)
                }
            }
            ROUND_DOWN_TO_GRID => {
                if distance >= 0 {
                    floor(distance).max(0)
                } else {
                    (-floor(-distance)).min(0)
                }
            }
            ROUND_UP_TO_GRID => {
                if distance >= 0 {
                    ceil(distance).max(0)
                } else {
                    (-ceil(-distance)).min(0)
                }
            }
            ROUND_SUPER => {
                if distance >= 0 {
                    let val = ((distance + (self.round_threshold - self.round_phase))
                        & -self.round_period)
                        + self.round_phase;
                    if val < 0 {
                        self.round_phase
                    } else {
                        val
                    }
                } else {
                    let val = -(((self.round_threshold - self.round_phase) - distance)
                        & -self.round_period)
                        - self.round_phase;
                    if val > 0 {
                        -self.round_phase
                    } else {
                        val
                    }
                }
            }
            ROUND_SUPER45 => {
                if distance >= 0 {
                    let val = (((distance + (self.round_threshold - self.round_phase))
                        / self.round_period)
                        * self.round_period)
                        + self.round_phase;
                    if val < 0 {
                        self.round_phase
                    } else {
                        val
                    }
                } else {
                    let val = -((((self.round_threshold - self.round_phase) - distance)
                        / self.round_period)
                        * self.round_period)
                        - self.round_phase;
                    if val > 0 {
                        -self.round_phase
                    } else {
                        val
                    }
                }
            }
            _ => distance,
        }
    }

    fn move_original(&mut self, zone: u8, point: usize, distance: i32) -> Option<()> {
        let fdotp = self.fdotp;
        let x = self.fv.x;
        let y = self.fv.y;
        let state = self.move_state;
        let z = self.zp_mut(zone);
        let p = z.original.get_mut(point)?;
        match state {
            1 => p.x += distance,
            2 => p.y += distance,
            _ => {
                if x != 0 {
                    p.x += muldiv(distance, x as i32, fdotp);
                }
                if y != 0 {
                    p.y += muldiv(distance, y as i32, fdotp);
                }
            }
        }
        Some(())
    }

    fn move_point(&mut self, zone: u8, point: usize, distance: i32) -> Option<()> {
        let legacy = self.v35;
        let bc = self.compat;
        let iupx = self.iupx;
        let iupy = self.iupy;
        let x = self.fv.x;
        let y = self.fv.y;
        let fdotp = self.fdotp;
        let state = self.move_state;
        let z = self.zp_mut(zone);
        let p = z.points.get_mut(point)?;
        let tag = z.tags.get_mut(point)?;
        match state {
            1 => {
                if legacy || !bc {
                    p.x += distance;
                }
                *tag |= TOUCH_X;
            }
            2 => {
                if !(!legacy && bc && iupx && iupy) {
                    p.y += distance;
                }
                *tag |= TOUCH_Y;
            }
            _ => {
                if x != 0 {
                    if legacy || !bc {
                        p.x += muldiv(distance, x as i32, fdotp);
                    }
                    *tag |= TOUCH_X;
                }
                if y != 0 {
                    if !(!legacy && bc && iupx && iupy) {
                        p.y += muldiv(distance, y as i32, fdotp);
                    }
                    *tag |= TOUCH_Y;
                }
            }
        }
        Some(())
    }

    #[inline(always)]
    fn project(&self, v1: Point, v2: Point) -> i32 {
        match self.project_state {
            1 => v1.x - v2.x,
            2 => v1.y - v2.y,
            _ => {
                let x = v1.x - v2.x;
                let y = v1.y - v2.y;
                dot14(x, y, self.pv.x as i32, self.pv.y as i32)
            }
        }
    }

    #[inline(always)]
    fn dual_project(&self, v1: Point, v2: Point) -> i32 {
        match self.project_state {
            1 => v1.x - v2.x,
            2 => v1.y - v2.y,
            _ => {
                let x = v1.x - v2.x;
                let y = v1.y - v2.y;
                dot14(x, y, self.dv.x as i32, self.dv.y as i32)
            }
        }
    }

    #[inline(always)]
    fn fast_project(&self, v: Point) -> i32 {
        self.project(v, Point::new(0, 0))
    }

    #[inline(always)]
    fn fast_dual_project(&self, v: Point) -> i32 {
        self.dual_project(v, Point::new(0, 0))
    }

    fn vectors_changed(&mut self) {
        if self.fv.x == 0x4000 {
            self.fdotp = self.pv.x as i32;
        } else if self.fv.y == 0x4000 {
            self.fdotp = self.pv.y as i32;
        } else {
            let px = self.pv.x as i32;
            let py = self.pv.y as i32;
            let fx = self.fv.x as i32;
            let fy = self.fv.y as i32;
            self.fdotp = (px * fx + py * fy) >> 14;
        }
        if self.pv.x == 0x4000 {
            self.project_state = 1;
        } else if self.pv.y == 0x4000 {
            self.project_state = 2;
        } else {
            self.project_state = 0;
        }
        if self.dv.x == 0x4000 {
            self.dual_project_state = 1;
        } else if self.dv.y == 0x4000 {
            self.dual_project_state = 2;
        } else {
            self.dual_project_state = 0;
        }
        self.move_state = 0;
        if self.fdotp == 0x4000 {
            if self.fv.x == 0x4000 {
                self.move_state = 1;
            } else if self.fv.y == 0x4000 {
                self.move_state = 2;
            }
        }
        if self.fdotp.abs() < 0x400 {
            self.fdotp = 0x4000;
        }
    }

    fn skip_instruction(&mut self, code: &[u8], pc: usize, next_pc: &mut usize) -> Option<bool> {
        let len = code.len();
        if pc < len {
            let opcode = *code.get(pc)?;
            let l = OPCODE_LEN[opcode as usize] as i32;
            let opcode_len;
            if l < 0 {
                opcode_len = (2 - l * *code.get(pc + 1)? as i32) as usize
            } else {
                opcode_len = l as usize;
            }
            if pc + opcode_len <= len {
                *next_pc = pc + opcode_len;
                return Some(true);
            }
        }
        None
    }

    fn compute_point_displacement(
        &mut self,
        opcode: u8,
        rp1: usize,
        rp2: usize,
    ) -> Option<(i32, i32, u8, usize)> {
        let (zone, index) = if (opcode & 1) != 0 {
            (self.zp0, rp1)
        } else {
            (self.zp1, rp2)
        };
        let z = self.zp(zone);
        let point = z.points.get(index)?;
        let original = z.original.get(index)?;
        let d = self.project(*point, *original);
        let x = muldiv(d, self.fv.x as i32, self.fdotp);
        let y = muldiv(d, self.fv.y as i32, self.fdotp);
        Some((x, y, zone, index))
    }

    fn move_zp2_point(&mut self, point: usize, dx: i32, dy: i32, touch: bool) -> Option<()> {
        let v35 = self.v35;
        let x = self.fv.x;
        let y = self.fv.y;
        let (iupx, iupy) = (self.iupx, self.iupy);
        let compat = self.compat;
        let z = self.zp2_mut();
        let p = z.points.get_mut(point)?;
        let tag = z.tags.get_mut(point)?;
        if x != 0 {
            if v35 || !compat {
                p.x += dx;
            }
            if touch {
                *tag |= TOUCH_X;
            }
        }
        if y != 0 {
            if !(!v35 && compat && iupx && iupy) {
                p.y += dy;
            }
            if touch {
                *tag |= TOUCH_Y;
            }
        }
        Some(())
    }

    fn normalize(&self, x: i32, y: i32, r: &mut Point) {
        use core::num::Wrapping;
        let (mut sx, mut sy) = (Wrapping(1i32), Wrapping(1i32));
        let mut ux = Wrapping(x as u32);
        let mut uy = Wrapping(y as u32);
        const ZERO: Wrapping<u32> = Wrapping(0);
        if x < 0 {
            ux = ZERO - ux;
            sx = -sx;
        }
        if y < 0 {
            uy = ZERO - uy;
            sy = -sy;
        }
        if ux == ZERO {
            r.x = x / 4;
            if uy.0 > 0 {
                r.y = (sy * Wrapping(0x10000) / Wrapping(4)).0;
            }
            return;
        }
        if uy == ZERO {
            r.y = y / 4;
            if ux.0 > 0 {
                r.x = (sx * Wrapping(0x10000) / Wrapping(4)).0;
            }
            return;
        }
        let mut len = if ux > uy {
            ux + (uy >> 1)
        } else {
            uy + (ux >> 1)
        };
        let mut shift = Wrapping(len.0.leading_zeros() as i32);
        shift -= Wrapping(15)
            + if len >= (Wrapping(0xAAAAAAAAu32) >> shift.0 as usize) {
                Wrapping(1)
            } else {
                Wrapping(0)
            };
        if shift.0 > 0 {
            let s = shift.0 as usize;
            ux <<= s;
            uy <<= s;
            len = if ux > uy {
                ux + (uy >> 1)
            } else {
                uy + (ux >> 1)
            };
        } else {
            let s = -shift.0 as usize;
            ux >>= s;
            uy >>= s;
            len >>= s;
        }
        let mut b = Wrapping(0x10000) - Wrapping(len.0 as i32);
        let x = Wrapping(ux.0 as i32);
        let y = Wrapping(uy.0 as i32);
        let mut z;
        let mut u;
        let mut v;
        loop {
            u = Wrapping((x + ((x * b) >> 16)).0 as u32);
            v = Wrapping((y + ((y * b) >> 16)).0 as u32);
            z = Wrapping(-((u * u + v * v).0 as i32)) / Wrapping(0x200);
            z = z * ((Wrapping(0x10000) + b) >> 8) / Wrapping(0x10000);
            b += z;
            if z <= Wrapping(0) {
                break;
            }
        }
        r.x = (Wrapping(u.0 as i32) * sx / Wrapping(4)).0;
        r.y = (Wrapping(v.0 as i32) * sy / Wrapping(4)).0;
    }
}

impl<'a> Hinter<'a> {
    fn execute<'b>(
        &mut self,
        state: &'b mut HinterState,
        programs: [&[u8]; 3],
        mut program: u8,
        composite: bool,
    ) -> Option<u32> {
        let mut code = programs[program as usize];
        if code.is_empty() {
            return Some(0);
        }
        let (v35, grayscale, subpixel, grayscale_cleartype) = match state.mode {
            HinterMode::Legacy => (true, true, false, false),
            HinterMode::GrayscaleSubpixel => (false, false, true, true),
            HinterMode::Subpixel => (false, false, true, false),
            HinterMode::Modern => (false, false, true, false),
        };
        self.v35 = v35;
        self.subpixel = subpixel;
        if state.mode == HinterMode::Modern {
            self.compat = true;
        } else if !v35 && subpixel {
            self.compat = (state.gs.instruct_control & 0x4) == 0;
        } else {
            self.compat = false;
        }
        self.compat = true;
        state.compat = self.compat;
        self.dv = Point::new(0x4000, 0);
        self.pv = self.dv;
        self.fv = self.dv;
        self.project_state = 0;
        self.move_state = 0;
        self.dual_project_state = 0;
        self.fdotp = 0x4000;
        self.zp0 = 1;
        self.zp1 = 1;
        self.zp2 = 1;
        self.round_state = ROUND_TO_GRID;
        self.round_threshold = 0;
        self.round_phase = 0;
        self.round_period = 64;
        self.iupx = false;
        self.iupy = false;
        self.vectors_changed();
        let mut stack_top = 0usize;
        let mut new_top: usize;
        let mut args_top: usize;
        let mut pc = 0usize;
        let mut count = 0u32;
        #[derive(Copy, Clone, Default)]
        struct CallRecord {
            caller_program: u8,
            caller_ip: usize,
            current_count: u32,
            definition: Function,
        }
        let mut callstack = [CallRecord::default(); 32];
        let mut callstack_top = 0;
        let callstack_len = callstack.len();
        let stack_size = self.stack.len();
        let mut rp0 = 0usize;
        let mut rp1 = 0usize;
        let mut rp2 = 0usize;
        let mut iloop = 1u32;
        loop {
            let opcode = *code.get(pc)?;
            let mut opcode_len = OPCODE_LEN[opcode as usize] as i32;
            if opcode_len < 0 {
                opcode_len = 2 - opcode_len * *code.get(pc + 1)? as i32;
            }
            let oplen = opcode_len as usize;
            let mut next_pc = pc + oplen;
            let pop_push = OPCODE_POP_PUSH[opcode as usize] as usize;
            let pop_count = pop_push >> 4;
            let push_count = pop_push & 0xF;
            if pop_count > stack_top {
                return None;
            }
            let args = stack_top - pop_count;
            args_top = args;
            new_top = args + push_count;
            if new_top > stack_size {
                return None;
            }

            if TRACE {
                // let name = OPCODE_NAME[opcode as usize];
                // for _ in 0..callstack_top {
                //     print!(".");
                // }
                // print!("{} [{}] {}", count, pc, name);
                // let pcnt = if stack_top < 8 { stack_top } else { 8 };
                // for i in 1..=pcnt {
                //     print!(" {}", self.stack[stack_top - i]);
                // }
                // println!("");
            }

            let a0 = args;
            let a1 = args + 1;
            let a2 = args + 2;

            match opcode {
                OP_SVTCA0..=OP_SFVTCA1 => {
                    let aa = ((opcode as i32 & 1) << 14) as i32;
                    let bb = aa ^ 0x4000;
                    if opcode < 4 {
                        self.pv = Point::new(aa, bb);
                        self.dv = self.pv;
                    }
                    if (opcode & 2) == 0 {
                        self.fv = Point::new(aa, bb);
                    }
                    self.vectors_changed();
                }
                OP_SPVTL0..=OP_SFVTL1 => {
                    let index1 = *self.stack.get(a1)? as usize;
                    let index2 = *self.stack.get(a0)? as usize;
                    let mut v = Point::new(0, 0);
                    let p1 = self.zp1().points.get(index2)?;
                    let p2 = self.zp2().points.get(index1)?;
                    let mut a = p1.x - p2.x;
                    let mut b = p1.y - p2.y;
                    let mut op = opcode;
                    if a == 0 && b == 0 {
                        a = 0x4000;
                        op = 0;
                    }
                    if (op & 1) != 0 {
                        let c = b;
                        b = a;
                        a = -c;
                    }
                    self.normalize(a, b, &mut v);
                    if opcode <= OP_SPVTL1 {
                        self.pv = v;
                        self.dv = v;
                    } else {
                        self.fv = v;
                    }
                    self.vectors_changed();
                }
                OP_SPVFS => {
                    let y = *self.stack.get(a1)? as i16 as i32;
                    let x = *self.stack.get(a0)? as i16 as i32;
                    let mut v = self.pv;
                    self.normalize(x, y, &mut v);
                    self.pv = v;
                    self.dv = v;
                    self.vectors_changed();
                }
                OP_SFVFS => {
                    let y = *self.stack.get(a1)? as i16 as i32;
                    let x = *self.stack.get(a0)? as i16 as i32;
                    let mut v = self.fv;
                    self.normalize(x, y, &mut v);
                    self.fv = v;
                    self.vectors_changed();
                }
                OP_GPV => {
                    *self.stack.get_mut(a0)? = self.pv.x;
                    *self.stack.get_mut(a1)? = self.pv.y;
                }
                OP_GFV => {
                    *self.stack.get_mut(a0)? = self.fv.x;
                    *self.stack.get_mut(a1)? = self.fv.y;
                }
                OP_SFVTPV => {
                    self.fv = self.pv;
                    self.vectors_changed();
                }
                OP_ISECT => {
                    let point = *self.stack.get(args)? as usize;
                    let a0 = *self.stack.get(args + 1)? as usize;
                    let a1 = *self.stack.get(args + 2)? as usize;
                    let b0 = *self.stack.get(args + 3)? as usize;
                    let b1 = *self.stack.get(args + 4)? as usize;
                    let (pa0, pa1) = {
                        let z = self.zp1();
                        (*z.points.get(a0)?, *z.points.get(a1)?)
                    };
                    let (pb0, pb1) = {
                        let z = self.zp0();
                        (*z.points.get(b0)?, *z.points.get(b1)?)
                    };
                    let dbx = pb1.x - pb0.x;
                    let dby = pb1.y - pb0.y;
                    let dax = pa1.x - pa0.x;
                    let day = pa1.y - pa0.y;
                    let dx = pb0.x - pa0.x;
                    let dy = pb0.y - pa0.y;
                    let discriminant = muldiv(dax, -dby, 0x40) + muldiv(day, dbx, 0x40);
                    let dp = muldiv(dax, dbx, 0x40) + muldiv(day, dby, 0x40);
                    if 19 * discriminant.abs() > dp.abs() {
                        let v = muldiv(dx, -dby, 0x40) + muldiv(dy, dbx, 0x40);
                        let x = muldiv(v, dax, discriminant);
                        let y = muldiv(v, day, discriminant);
                        let p = self.zp2_mut().points.get_mut(point)?;
                        p.x = pa0.x + x;
                        p.y = pa0.y + y;
                    } else {
                        let p = self.zp2_mut().points.get_mut(point)?;
                        p.x = (pa0.x + pa1.x + pb0.x + pb1.x) / 4;
                        p.y = (pa0.y + pa1.y + pb0.y + pb1.y) / 4;
                    }
                    *self.zp2_mut().tags.get_mut(point)? |= TOUCH_X | TOUCH_Y;
                }
                OP_SRP0 => rp0 = *self.stack.get(a0)? as usize,
                OP_SRP1 => rp1 = *self.stack.get(a0)? as usize,
                OP_SRP2 => rp2 = *self.stack.get(a0)? as usize,
                OP_SZP0 => {
                    let z = *self.stack.get(a0)? as u8;
                    if z > 1 {
                        return None;
                    } else {
                        self.zp0 = z;
                    }
                }
                OP_SZP1 => {
                    let z = *self.stack.get(a0)? as u8;
                    if z > 1 {
                        return None;
                    } else {
                        self.zp1 = z;
                    }
                }
                OP_SZP2 => {
                    let z = *self.stack.get(a0)? as u8;
                    if z > 1 {
                        return None;
                    } else {
                        self.zp2 = z;
                    }
                }
                OP_SZPS => {
                    let z = *self.stack.get(a0)? as u8;
                    if z > 1 {
                        return None;
                    } else {
                        self.zp0 = z;
                        self.zp1 = z;
                        self.zp2 = z;
                    }
                }
                OP_SLOOP => {
                    let c = *self.stack.get(a0)?;
                    if c < 0 {
                        return None;
                    } else {
                        iloop = (c as u32).min(0xFFFF);
                    }
                }
                OP_RTG => self.round_state = ROUND_TO_GRID,
                OP_RTHG => self.round_state = ROUND_TO_HALF_GRID,
                OP_SMD => state.gs.min_distance = *self.stack.get(a0)?,
                OP_ELSE => {
                    let mut n = 1;
                    next_pc = pc;
                    while n != 0 && self.skip_instruction(code, next_pc, &mut next_pc)? {
                        match code[next_pc] {
                            OP_IF => n += 1,
                            OP_EIF => n -= 1,
                            _ => {}
                        }
                    }
                    next_pc += 1;
                }
                OP_SCVTCI => state.gs.control_value_cutin = *self.stack.get(a0)?,
                OP_SSWCI => state.gs.single_width_cutin = *self.stack.get(a0)?,
                OP_SSW => state.gs.single_width = *self.stack.get(a0)?,
                OP_DUP => *self.stack.get_mut(a1)? = *self.stack.get(a0)?,
                OP_POP => {}
                OP_CLEAR => new_top = 0,
                OP_SWAP => {
                    let s = &mut *self.stack;
                    let t = *s.get(a0)?;
                    *s.get_mut(a0)? = *s.get(a1)?;
                    *s.get_mut(a1)? = t;
                }
                OP_DEPTH => *self.stack.get_mut(a0)? = stack_top as i32,
                OP_CINDEX => {
                    let index = *self.stack.get(a0)? as usize;
                    if a0 == 0 || index > a0 {
                        return None;
                    } else {
                        let v = *self.stack.get(a0 - index)?;
                        *self.stack.get_mut(a0)? = v;
                    }
                }
                OP_MINDEX => {
                    let index = *self.stack.get(a0)? as usize;
                    if a0 == 0 || index > a0 {
                        return None;
                    } else {
                        let s = &mut *self.stack;
                        let e = *s.get(a0 - index)?;
                        for i in (a0 - index)..(a0 - 1) {
                            let v = *s.get(i + 1)?;
                            *s.get_mut(i)? = v;
                        }
                        *s.get_mut(a0 - 1)? = e;
                    }
                }
                OP_ALIGNPTS => {
                    let p1 = *self.stack.get(a0)? as usize;
                    let p2 = *self.stack.get(a1)? as usize;
                    let distance =
                        self.project(*self.zp0().points.get(p2)?, *self.zp1().points.get(p1)?) / 2;
                    self.move_point(self.zp1, p1, distance);
                    self.move_point(self.zp0, p2, -distance);
                }
                OP_UTP => {
                    let point = *self.stack.get(a0)? as usize;
                    let mut mask = 0xFF;
                    if self.fv.x != 0 {
                        mask &= !TOUCH_X;
                    }
                    if self.fv.y != 0 {
                        mask &= !TOUCH_Y;
                    }
                    *self.zp0_mut().tags.get_mut(point)? &= mask;
                }
                OP_LOOPCALL | OP_CALL => {
                    let (n, count) = if opcode == OP_LOOPCALL {
                        (*self.stack.get(a1)? as usize, *self.stack.get(a0)?)
                    } else {
                        (*self.stack.get(a0)? as usize, 1)
                    };
                    if callstack_top >= callstack_len {
                        return None;
                    }
                    if count > 0 {
                        let def = self.fdefs.get(n)?;
                        if !def.active {
                            return None;
                        }
                        let rec = CallRecord {
                            caller_program: program,
                            caller_ip: pc + 1,
                            current_count: count as u32,
                            definition: *def,
                        };
                        callstack[callstack_top] = rec;
                        callstack_top += 1;
                        program = def.program;
                        code = programs[program as usize];
                        next_pc = def.offset as usize;
                    }
                }
                OP_FDEF => {
                    let n = *self.stack.get(a0)? as usize;
                    if program == 2 || n >= self.fdefs.len() {
                        return None;
                    }
                    let mut def = *self.fdefs.get(n)?;
                    def.active = true;
                    def.program = program;
                    def.offset = (pc + 1) as u32;
                    def.end = def.offset;
                    next_pc = pc;
                    while self.skip_instruction(code, next_pc, &mut next_pc)? {
                        match code[next_pc] {
                            0x89 | 0x2C => {
                                return None;
                            }
                            0x2D => {
                                def.end = next_pc as u32;
                                self.fdefs[n] = def;
                                break;
                            }
                            _ => {}
                        }
                    }
                    next_pc += 1;
                }
                OP_ENDF => {
                    if callstack_top == 0 {
                        return None;
                    }
                    let rec = callstack.get_mut(callstack_top - 1)?;
                    if rec.current_count > 1 {
                        rec.current_count -= 1;
                        next_pc = rec.definition.offset as usize;
                    } else {
                        program = rec.caller_program;
                        code = *programs.get(program as usize)?;
                        next_pc = rec.caller_ip;
                        callstack_top -= 1;
                    }
                }
                OP_MDAP0 | OP_MDAP1 => {
                    let point = *self.stack.get(a0)? as usize;
                    let mut distance = 0;
                    if (opcode & 1) != 0 {
                        let c = self.fast_project(*self.zp0().points.get(point)?);
                        distance = self.round(c) - c;
                    }
                    self.move_point(self.zp0, point, distance)?;
                    rp0 = point;
                    rp1 = point;
                }
                OP_IUP0 | OP_IUP1 => {
                    let is_x = (opcode & 1) != 0;
                    let mut run = !self.glyph.contours.is_empty();
                    if !self.v35 && self.compat {
                        if self.iupx && self.iupy {
                            run = false;
                        }
                        if is_x {
                            self.iupx = true;
                        } else {
                            self.iupy = true;
                        }
                    }
                    if run {
                        let mask = if is_x { TOUCH_X } else { TOUCH_Y };
                        let mut point = 0;
                        for i in 0..self.glyph.contours.len() {
                            let mut end_point = *self.glyph.contours.get(i)? as usize;
                            let first_point = point;
                            if end_point >= self.glyph.points.len() {
                                end_point = self.glyph.points.len() - 1;
                            }
                            while point <= end_point && (*self.glyph.tags.get(point)? & mask) == 0 {
                                point += 1;
                            }
                            if point <= end_point {
                                let first_touched = point;
                                let mut cur_touched = point;
                                point += 1;
                                while point <= end_point {
                                    if (*self.glyph.tags.get(point)? & mask) != 0 {
                                        self.glyph.interpolate(
                                            is_x,
                                            cur_touched + 1,
                                            point - 1,
                                            cur_touched,
                                            point,
                                        )?;
                                        cur_touched = point;
                                    }
                                    point += 1;
                                }
                                if cur_touched == first_touched {
                                    self.glyph
                                        .shift(is_x, first_point, end_point, cur_touched)?;
                                } else {
                                    self.glyph.interpolate(
                                        is_x,
                                        cur_touched + 1,
                                        end_point,
                                        cur_touched,
                                        first_touched,
                                    )?;
                                    if first_touched > 0 {
                                        self.glyph.interpolate(
                                            is_x,
                                            first_point,
                                            first_touched - 1,
                                            cur_touched,
                                            first_touched,
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                }
                OP_SHP0 | OP_SHP1 => {
                    if stack_top < iloop as usize {
                        return None;
                    }
                    let (dx, dy, _, _) = self.compute_point_displacement(opcode, rp1, rp2)?;
                    while iloop > 0 {
                        args_top -= 1;
                        let index = *self.stack.get(args_top)? as usize;
                        self.move_zp2_point(index, dx, dy, true)?;
                        iloop -= 1;
                    }
                    iloop = 1;
                    new_top = args_top;
                }
                OP_SHC0 | OP_SHC1 => {
                    let contour = *self.stack.get(a0)? as usize;
                    let bound = if self.zp2 == 0 {
                        1
                    } else {
                        self.zp2().contours.len()
                    };
                    if contour >= bound {
                        return None;
                    }
                    let (dx, dy, zone, index) =
                        self.compute_point_displacement(opcode, rp1, rp2)?;
                    let mut start = 0;
                    if contour != 0 {
                        let z = self.zp2();
                        start = *z.contours.get(contour - 1)? as usize + 1;
                    }
                    let limit;
                    if self.zp2 == 0 {
                        limit = self.zp2().points.len();
                    } else {
                        let z = self.zp2();
                        limit = *z.contours.get(contour)? as usize + 1;
                    }
                    for i in start..limit {
                        if zone != self.zp2 || index != i {
                            self.move_zp2_point(i, dx, dy, true)?;
                        }
                    }
                }
                OP_SHZ0 | OP_SHZ1 => {
                    if *self.stack.get(a0)? >= 2 {
                        return None;
                    }
                    let (dx, dy, zone, index) =
                        self.compute_point_displacement(opcode, rp1, rp2)?;
                    let limit = if self.zp2 == 0 {
                        self.zp2().points.len()
                    } else if self.zp2 == 1 && !self.zp2().contours.is_empty() {
                        let z = self.zp2();
                        *z.contours.get(z.contours.len() - 1)? as usize + 1
                    } else {
                        0
                    };
                    for i in 0..limit {
                        if zone != self.zp2 || i != index {
                            self.move_zp2_point(i, dx, dy, false)?;
                        }
                    }
                }
                OP_SHPIX => {
                    if stack_top < iloop as usize + 1 {
                        return None;
                    }
                    let in_twilight = self.zp0 == 0 || self.zp1 == 0 || self.zp2 == 0;
                    let a = *self.stack.get(a0)?;
                    let dx = mul14(a, self.fv.x as i32);
                    let dy = mul14(a, self.fv.y as i32);
                    while iloop > 0 {
                        args_top -= 1;
                        let point = *self.stack.get(args_top)? as usize;
                        if !self.v35 && self.compat {
                            if in_twilight
                                || (!(self.iupx && self.iupy)
                                    && ((composite && self.fv.y != 0)
                                        || (*self.zp2().tags.get(point)? & TOUCH_Y != 0)))
                            {
                                self.move_zp2_point(point, dx, dy, true)?;
                            }
                        } else {
                            self.move_zp2_point(point, dx, dy, true)?;
                        }
                        iloop -= 1;
                    }
                    iloop = 1;
                    new_top = args_top;
                }
                OP_IP => {
                    if stack_top < iloop as usize {
                        return None;
                    }
                    let in_twilight = self.zp0 == 0 || self.zp1 == 0 || self.zp2 == 0;
                    let orus_base = if in_twilight {
                        *self.zp0().original.get(rp1)?
                    } else {
                        *self.zp0().unscaled.get(rp1)?
                    };
                    let cur_base = *self.zp0().points.get(rp1)?;
                    let old_range = if in_twilight {
                        self.dual_project(*self.zp1().original.get(rp2)?, orus_base)
                    } else {
                        self.dual_project(*self.zp1().unscaled.get(rp2)?, orus_base)
                    };
                    let cur_range = self.project(*self.zp1().points.get(rp2)?, cur_base);
                    while iloop > 0 {
                        iloop -= 1;
                        args_top -= 1;
                        let point = *self.stack.get(args_top)? as usize;
                        let original_distance = if in_twilight {
                            self.dual_project(*self.zp2().original.get(point)?, orus_base)
                        } else {
                            self.dual_project(*self.zp2().unscaled.get(point)?, orus_base)
                        };
                        let cur_distance = self.project(*self.zp2().points.get(point)?, cur_base);
                        let mut new_distance = 0;
                        if original_distance != 0 {
                            if old_range != 0 {
                                new_distance = muldiv(original_distance, cur_range, old_range);
                            } else {
                                new_distance = original_distance;
                            }
                        }
                        self.move_point(self.zp2, point, new_distance - cur_distance)?;
                    }
                    iloop = 1;
                    new_top = args_top;
                }
                OP_MSIRP0 | OP_MSIRP1 => {
                    let point = *self.stack.get(args)? as usize;
                    if self.zp1 == 0 {
                        *self.zp1_mut().points.get_mut(point)? = *self.zp0().original.get(rp0)?;
                        self.move_original(self.zp1, point, *self.stack.get(args + 1)?)?;
                        *self.zp1_mut().points.get_mut(point)? = *self.zp1().original.get(point)?;
                    }
                    let d =
                        self.project(*self.zp1().points.get(point)?, *self.zp0().points.get(rp0)?);
                    let a = *self.stack.get(args + 1)?;
                    self.move_point(self.zp1, point, a.wrapping_sub(d))?;
                    rp1 = rp0;
                    rp2 = point;
                    if (opcode & 1) != 0 {
                        rp0 = point;
                    }
                }
                OP_ALIGNRP => {
                    if stack_top < iloop as usize {
                        return None;
                    }
                    while iloop > 0 {
                        args_top -= 1;
                        let point = *self.stack.get(args_top)? as usize;
                        let distance = self
                            .project(*self.zp1().points.get(point)?, *self.zp0().points.get(rp0)?);
                        self.move_point(self.zp1, point, -distance)?;
                        iloop -= 1;
                    }
                    iloop = 1;
                    new_top = args_top;
                }
                OP_RTDG => self.round_state = ROUND_TO_DOUBLE_GRID,
                OP_MIAP0 | OP_MIAP1 => {
                    let point = *self.stack.get(a0)? as usize;
                    let cvt_entry = *self.stack.get(a1)? as usize;
                    let mut distance = *self.cvt.get(cvt_entry)?;
                    if self.zp0 == 0 {
                        let f = self.fv;
                        let z = self.zp0_mut();
                        let p = z.points.get_mut(point)?;
                        let o = z.original.get_mut(point)?;
                        o.x = mul14(distance, f.x as i32);
                        o.y = mul14(distance, f.y as i32);
                        *p = *o;
                    }
                    let original_distance = self.fast_project(*self.zp0().points.get(point)?);
                    if (opcode & 1) != 0 {
                        let delta = (distance - original_distance).abs();
                        if delta > state.gs.control_value_cutin {
                            distance = original_distance;
                        }
                        distance = self.round(distance);
                    }
                    self.move_point(self.zp0, point, distance - original_distance)?;
                    rp0 = point;
                    rp1 = point;
                }
                OP_NPUSHB => {
                    let count = code[pc + 1] as usize;
                    for (sp, cp) in self
                        .stack
                        .get_mut(a0..a0 + count)?
                        .iter_mut()
                        .zip(code.get(pc + 2..)?)
                    {
                        *sp = *cp as u32 as i32;
                    }
                    new_top += count;
                }
                OP_NPUSHW => {
                    let count = code[pc + 1] as usize;
                    for (sp, cp) in self
                        .stack
                        .get_mut(a0..a0 + count)?
                        .iter_mut()
                        .zip(code.get(pc + 2..)?.chunks(2))
                    {
                        let word = ((*cp.get(0)? as u16) << 8) | *cp.get(1)? as u16;
                        *sp = word as i16 as i32;
                    }
                    new_top += count;
                }
                OP_WS => {
                    let index = *self.stack.get(a0)? as usize;
                    *self.store.get_mut(index)? = *self.stack.get(a1)?;
                }
                OP_RS => {
                    let sp = self.stack.get_mut(a0)?;
                    let index = *sp as usize;
                    *sp = *self.store.get(index)?;
                }
                OP_WCVTP => {
                    let index = *self.stack.get(a0)? as usize;
                    *self.cvt.get_mut(index)? = *self.stack.get(a1)?;
                }
                OP_WCVTF => {
                    let index = *self.stack.get(a0)? as usize;
                    *self.cvt.get_mut(index)? = mul(*self.stack.get(a1)?, self.scale);
                }
                OP_RCVT => {
                    let sp = self.stack.get_mut(a0)?;
                    let index = *sp as usize;
                    *sp = *self.cvt.get(index)?;
                }
                OP_GC0 | OP_GC1 => {
                    let index = *self.stack.get(a0)? as usize;
                    let r = if (opcode & 1) != 0 {
                        self.fast_dual_project(*self.zp2().original.get(index)?)
                    } else {
                        self.fast_project(*self.zp2().points.get(index)?)
                    };
                    *self.stack.get_mut(a0)? = r;
                }
                OP_SCFS => {
                    let index = *self.stack.get(a0)? as usize;
                    let a = self.fast_project(*self.zp2().points.get(index)?);
                    self.move_point(self.zp2, index, self.stack.get(a1)?.wrapping_sub(a))?;
                    if self.zp2 == 0 {
                        *self.twilight.original.get_mut(index)? =
                            *self.twilight.points.get(index)?;
                    }
                }
                OP_MD0 | OP_MD1 => {
                    let a = *self.stack.get(a1)? as usize;
                    let b = *self.stack.get(a0)? as usize;
                    let d = if (opcode & 1) != 0 {
                        let v1 = self.zp0().points.get(b)?;
                        let v2 = self.zp1().points.get(a)?;
                        self.project(*v1, *v2)
                    } else if self.zp0 == 0 || self.zp1 == 0 {
                        let v1 = self.zp0().original.get(b)?;
                        let v2 = self.zp1().original.get(a)?;
                        self.dual_project(*v1, *v2)
                    } else {
                        let v1 = self.zp0().unscaled.get(b)?;
                        let v2 = self.zp1().unscaled.get(a)?;
                        mul(self.dual_project(*v1, *v2), self.yscale)
                    };
                    *self.stack.get_mut(a0)? = d;
                }
                OP_MPPEM => {
                    *self.stack.get_mut(a0)? = self.ppem as i32;
                }
                OP_MPS => {
                    *self.stack.get_mut(a0)? = if self.v35 {
                        self.ppem as i32
                    } else {
                        self.point_size
                    };
                }
                OP_FLIPON => state.gs.auto_flip = true,
                OP_FLIPOFF => state.gs.auto_flip = false,
                OP_DEBUG => {}
                OP_LT => {
                    *self.stack.get_mut(a0)? = (*self.stack.get(a0)? < *self.stack.get(a1)?) as i32
                }
                OP_LTEQ => {
                    *self.stack.get_mut(a0)? = (*self.stack.get(a0)? <= *self.stack.get(a1)?) as i32
                }
                OP_GT => {
                    *self.stack.get_mut(a0)? = (*self.stack.get(a0)? > *self.stack.get(a1)?) as i32
                }
                OP_GTEQ => {
                    *self.stack.get_mut(a0)? = (*self.stack.get(a0)? >= *self.stack.get(a1)?) as i32
                }
                OP_EQ => {
                    *self.stack.get_mut(a0)? = (*self.stack.get(a0)? == *self.stack.get(a1)?) as i32
                }
                OP_NEQ => {
                    *self.stack.get_mut(a0)? = (*self.stack.get(a0)? != *self.stack.get(a1)?) as i32
                }
                OP_ODD => {
                    *self.stack.get_mut(a0)? = (self.round(*self.stack.get(a0)?) & 127 == 64) as i32
                }
                OP_EVEN => {
                    *self.stack.get_mut(a0)? = (self.round(*self.stack.get(a0)?) & 127 == 0) as i32
                }
                OP_IF => {
                    if *self.stack.get(a0)? == 0 {
                        let mut n = 1;
                        let mut out = false;
                        next_pc = pc;
                        while !out && self.skip_instruction(code, next_pc, &mut next_pc)? {
                            match *code.get(next_pc)? {
                                OP_IF => n += 1,
                                OP_ELSE => out = n == 1,
                                OP_EIF => {
                                    n -= 1;
                                    out = n == 0;
                                }
                                _ => {}
                            }
                        }
                        next_pc += 1;
                    }
                }
                OP_EIF => {}
                OP_AND => {
                    *self.stack.get_mut(a0)? =
                        (*self.stack.get(a0)? != 0 && *self.stack.get(a1)? != 0) as i32
                }
                OP_OR => {
                    *self.stack.get_mut(a0)? =
                        (*self.stack.get(a0)? != 0 || *self.stack.get(a1)? != 0) as i32
                }
                OP_NOT => *self.stack.get_mut(a0)? = (*self.stack.get(a0)? == 0) as i32,
                OP_SDB => state.gs.delta_base = *self.stack.get(a0)? as u16,
                OP_SDS => state.gs.delta_shift = (*self.stack.get(a0)?).min(6) as u16,
                OP_ADD => *self.stack.get_mut(a0)? += *self.stack.get(a1)?,
                OP_SUB => *self.stack.get_mut(a0)? -= *self.stack.get(a1)?,
                OP_DIV => {
                    let d = *self.stack.get(a1)?;
                    if d == 0 {
                        return None;
                    }
                    let sp = self.stack.get_mut(a0)?;
                    *sp = muldiv_no_round(*sp, 64, d);
                }
                OP_MUL => {
                    *self.stack.get_mut(a0)? =
                        muldiv(*self.stack.get(a0)?, *self.stack.get(a1)?, 64)
                }
                OP_ABS => *self.stack.get_mut(a0)? = (*self.stack.get(a0)?).abs(),
                OP_NEG => *self.stack.get_mut(a0)? = -*self.stack.get(a0)?,
                OP_FLOOR => *self.stack.get_mut(a0)? = floor(*self.stack.get(a0)?),
                OP_CEILING => *self.stack.get_mut(a0)? = ceil(*self.stack.get(a0)?),
                OP_ROUND00..=OP_ROUND11 => {
                    *self.stack.get_mut(a0)? = self.round(*self.stack.get(a0)?)
                }
                OP_NROUND00..=OP_NROUND11 => {}
                OP_DELTAP1 | OP_DELTAP2 | OP_DELTAP3 => {
                    let p = self.ppem as u32;
                    let nump = *self.stack.get(a0)? as u32;
                    let bias = match opcode {
                        OP_DELTAP2 => 16,
                        OP_DELTAP3 => 32,
                        _ => 0,
                    } + state.gs.delta_base as u32;
                    for _ in 1..=nump {
                        if args_top < 2 {
                            return None;
                        }
                        args_top -= 2;
                        let a = *self.stack.get(args_top + 1)? as usize;
                        if a >= self.zp0().points.len() {
                            continue;
                        }
                        let mut b = *self.stack.get(args_top)?;
                        let mut c = (b as u32 & 0xF0) >> 4;
                        c += bias;
                        if p == c {
                            b = (b & 0xF) - 8;
                            if b >= 0 {
                                b += 1;
                            }
                            b *= 1 << (6 - state.gs.delta_shift as i32);
                            if !self.v35 && self.compat {
                                if !(self.iupx && self.iupy)
                                    && ((composite && self.fv.y != 0)
                                        || (*self.zp0().tags.get(a)? & TOUCH_Y) != 0)
                                {
                                    self.move_point(self.zp0, a, b)?;
                                }
                            } else {
                                self.move_point(self.zp0, a, b)?;
                            }
                        }
                    }
                    new_top = args_top;
                }
                OP_DELTAC1 | OP_DELTAC2 | OP_DELTAC3 => {
                    let p = self.ppem as u32;
                    let nump = *self.stack.get(args)? as u32;
                    let bias = match opcode {
                        OP_DELTAC2 => 16,
                        OP_DELTAC3 => 32,
                        _ => 0,
                    } + state.gs.delta_base as u32;
                    for _ in 1..=nump {
                        if args_top < 2 {
                            return None;
                        }
                        args_top -= 2;
                        let a = *self.stack.get(args_top + 1)? as usize;
                        let mut b = *self.stack.get(args_top)?;
                        let mut c = (b as u32 & 0xF0) >> 4;
                        c += bias;
                        if p == c {
                            b = (b & 0xF) - 8;
                            if b >= 0 {
                                b += 1;
                            }
                            b *= 1 << (6 - state.gs.delta_shift as i32);
                            *self.cvt.get_mut(a)? += b;
                        }
                    }
                    new_top = args_top;
                }
                OP_SROUND | OP_S45ROUND => {
                    let selector = *self.stack.get(a0)?;
                    let grid_period = if opcode == OP_SROUND {
                        self.round_state = ROUND_SUPER;
                        0x4000
                    } else {
                        self.round_state = ROUND_SUPER45;
                        0x2D41
                    };
                    match selector & 0xC0 {
                        0 => self.round_period = grid_period / 2,
                        0x40 => self.round_period = grid_period,
                        0x80 => self.round_period = grid_period * 2,
                        0xC0 => self.round_period = grid_period,
                        _ => {}
                    }
                    match selector & 0x30 {
                        0 => self.round_phase = 0,
                        0x10 => self.round_phase = self.round_period / 4,
                        0x20 => self.round_phase = self.round_period / 2,
                        0x30 => self.round_phase = self.round_period * 3 / 4,
                        _ => {}
                    }
                    if (selector & 0x0F) == 0 {
                        self.round_threshold = self.round_period - 1;
                    } else {
                        self.round_threshold = ((selector & 0x0F) - 4) * self.round_period / 8;
                    }
                    self.round_period >>= 8;
                    self.round_phase >>= 8;
                    self.round_threshold >>= 8;
                }
                OP_JMPR | OP_JROT | OP_JROF => {
                    let cond = match opcode {
                        OP_JROT => *self.stack.get(a1)? != 0,
                        OP_JROF => *self.stack.get(a1)? == 0,
                        _ => true,
                    };
                    if cond {
                        let o = *self.stack.get(a0)?;
                        if o == 0 && args == 0 {
                            return None;
                        }
                        if o < 0 {
                            next_pc = pc - (-o) as usize;
                        } else {
                            next_pc = pc + o as usize;
                        }
                        if callstack_top > 0
                            && next_pc > callstack[callstack_top - 1].definition.end as usize
                        {
                            return None;
                        }
                    }
                }
                OP_ROFF => self.round_state = ROUND_OFF,
                OP_RUTG => self.round_state = ROUND_UP_TO_GRID,
                OP_RDTG => self.round_state = ROUND_DOWN_TO_GRID,
                OP_SANGW => {}
                OP_AA => {}
                OP_FLIPPT => {
                    if !self.v35 && self.compat && self.iupx && self.iupy {
                        // nothing
                    } else if stack_top < iloop as usize {
                        return None;
                    } else {
                        while iloop > 0 {
                            args_top -= 1;
                            let point = *self.stack.get(args_top)? as usize;
                            *self.glyph.tags.get_mut(point)? ^= 1;
                            iloop -= 1;
                        }
                    }
                    iloop = 1;
                    new_top = args_top;
                }
                OP_FLIPRGON | OP_FLIPRGOFF => {
                    if !self.v35 && self.compat && self.iupx && self.iupy {
                        // nothing
                    } else {
                        let a = *self.stack.get(a1)? as usize;
                        let b = *self.stack.get(a0)? as usize;
                        if b > a {
                            return None;
                        }
                        if opcode == OP_FLIPRGON {
                            for tag in self.glyph.tags.get_mut(b..=a)? {
                                *tag |= 1;
                            }
                        } else {
                            for tag in self.glyph.tags.get_mut(b..=a)? {
                                *tag &= !1;
                            }
                        }
                    }
                }
                OP_SCANCTRL => {
                    let a = *self.stack.get(a0)? as u16;
                    let b = a & 0xFF;
                    let scan_control = &mut state.gs.scan_control;
                    if b == 0xFF {
                        *scan_control = true;
                    } else if b == 0 {
                        *scan_control = false;
                    } else {
                        if (a & 0x100) != 0 && self.ppem <= b {
                            *scan_control = true;
                        }
                        if (a & 0x200) != 0 && self.rotated {
                            *scan_control = true;
                        }
                        if (a & 0x800) != 0 && self.ppem > b {
                            *scan_control = false;
                        }
                        if (a & 0x1000) != 0 && self.rotated {
                            *scan_control = false;
                        }
                    }
                }
                OP_SDPVTL0 | OP_SDPVTL1 => {
                    let mut op = opcode;
                    let p1 = *self.stack.get(a1)? as usize;
                    let p2 = *self.stack.get(a0)? as usize;
                    let mut a;
                    let mut b;
                    {
                        let v1 = self.zp1().original.get(p2)?;
                        let v2 = self.zp2().original.get(p1)?;
                        a = v1.x - v2.x;
                        b = v1.y - v2.y;
                        if a == 0 && b == 0 {
                            a = 0x4000;
                            op = 0;
                        }
                    }
                    if (op & 1) != 0 {
                        let c = b;
                        b = a;
                        a = -c;
                    }
                    let mut v = self.dv;
                    self.normalize(a, b, &mut v);
                    self.dv = v;
                    {
                        let v1 = self.zp1().points.get(p2)?;
                        let v2 = self.zp2().points.get(p1)?;
                        a = v1.x - v2.x;
                        b = v1.y - v2.y;
                        if a == 0 && b == 0 {
                            a = 0x4000;
                            op = 0;
                        }
                    }
                    if (op & 1) != 0 {
                        let c = b;
                        b = a;
                        a = -c;
                    }
                    let mut v = self.pv;
                    self.normalize(a, b, &mut v);
                    self.pv = v;
                    self.vectors_changed();
                }
                OP_GETINFO => {
                    let a = *self.stack.get(a0)?;
                    let mut k = 0;
                    if (a & 1) != 0 {
                        k = if self.v35 { 35 } else { 42 };
                    }
                    if (a & 2) != 0 && self.rotated {
                        k |= 1 << 8;
                    }
                    if (a & 8) != 0 && !self.coords.is_empty() {
                        k |= 1 << 10;
                    }
                    if (a & 32) != 0 && grayscale {
                        k |= 1 << 12;
                    }
                    if !self.v35 && self.subpixel {
                        if (a & 64) != 0 {
                            k |= 1 << 13;
                        }
                        // if (a & 256) != 0 && false
                        // /* self.vertical_lcd */
                        // {
                        //     k |= 1 << 15;
                        // }
                        if (a & 1024) != 0 {
                            k |= 1 << 17;
                        }
                        // if (a & 2048) != 0 && self.subpixel {
                        //     k |= 1 << 18;
                        // }
                        if (a & 4096) != 0 && grayscale_cleartype {
                            k |= 1 << 19;
                        }
                    }
                    *self.stack.get_mut(a0)? = k;
                }
                OP_IDEF => {
                    if program == 2 {
                        return None;
                    }
                    let n = *self.stack.get(args)? as usize;
                    let mut index = !0;
                    for i in 0..self.idefs.len() {
                        if !self.idefs[i].active {
                            index = i;
                            break;
                        }
                    }
                    if index == !0 {
                        return None;
                    }
                    let mut def = self.idefs[index];
                    def.program = program;
                    def.opcode = n as u16;
                    def.offset = pc as u32 + 1;
                    def.active = true;
                    next_pc = pc;
                    while self.skip_instruction(code, next_pc, &mut next_pc)? {
                        match *code.get(next_pc)? {
                            0x89 | 0x2C => {
                                return None;
                            }
                            0x2D => {
                                def.end = next_pc as u32;
                                self.idefs[index] = def;
                                break;
                            }
                            _ => {}
                        }
                    }
                    next_pc += 1;
                }
                OP_ROLL => {
                    let s = &mut *self.stack;
                    let (a, b, c) = (*s.get(a2)?, *s.get(a1)?, *s.get(a0)?);
                    *s.get_mut(a2)? = c;
                    *s.get_mut(a1)? = a;
                    *s.get_mut(a0)? = b;
                }
                OP_MAX => {
                    *self.stack.get_mut(a0)? = (*self.stack.get(a0)?).max(*self.stack.get(a1)?)
                }
                OP_MIN => {
                    *self.stack.get_mut(a0)? = (*self.stack.get(a0)?).min(*self.stack.get(a1)?)
                }
                OP_SCANTYPE => {
                    let a = *self.stack.get(a0)?;
                    if a >= 0 {
                        state.gs.scan_type = a & 0xFFFF;
                    }
                }
                OP_INSTCTRL => {
                    let a = *self.stack.get(a1)? as u32;
                    let b = *self.stack.get(a0)? as u32;
                    let af = 1 << (a - 1);
                    if !(1..=3).contains(&a) || (b != 0 && b != af) {
                        // nothing
                    } else {
                        state.gs.instruct_control &= !(af as u8);
                        state.gs.instruct_control |= b as u8;
                        if a == 3 && !self.v35 && state.mode != HinterMode::Modern {
                            self.compat = b != 4;
                        }
                    }
                }
                OP_PUSHB000..=OP_PUSHB111 => {
                    let count = (opcode - 0xB0 + 1) as usize;
                    for (sp, cp) in (self.stack.get_mut(a0..a0 + count)?)
                        .iter_mut()
                        .zip(code.get(pc + 1..)?)
                    {
                        *sp = *cp as u32 as i32;
                    }
                }
                OP_PUSHW000..=OP_PUSHW111 => {
                    let count = (opcode - 0xB8 + 1) as usize;
                    for (sp, cp) in (self.stack.get_mut(a0..a0 + count)?)
                        .iter_mut()
                        .zip(code.get(pc + 1..)?.chunks(2))
                    {
                        let word = ((cp[0] as u16) << 8) | cp[1] as u16;
                        *sp = word as i16 as i32;
                    }
                }
                OP_MDRP00000..=OP_MDRP11111 => {
                    let point = *self.stack.get(args)? as usize;
                    let mut original_distance;
                    if self.zp0 == 0 || self.zp1 == 0 {
                        original_distance = self.dual_project(
                            *self.zp1().original.get(point)?,
                            *self.zp0().original.get(rp0)?,
                        );
                    } else {
                        let v1 = self.zp1().unscaled.get(point)?;
                        let v2 = self.zp0().unscaled.get(rp0)?;
                        original_distance = self.dual_project(*v1, *v2);
                        original_distance = mul(original_distance, self.yscale);
                    }
                    let cutin = state.gs.single_width_cutin;
                    let value = state.gs.single_width;
                    if cutin > 0
                        && original_distance < value + cutin
                        && original_distance > value - cutin
                    {
                        original_distance = if original_distance >= 0 {
                            value
                        } else {
                            -value
                        };
                    }
                    let mut distance = if (opcode & 4) != 0 {
                        self.round(original_distance)
                    } else {
                        original_distance
                    };
                    let min_distance = state.gs.min_distance;
                    if (opcode & 8) != 0 {
                        if original_distance >= 0 {
                            if distance < min_distance {
                                distance = min_distance;
                            }
                        } else if distance > -min_distance {
                            distance = -min_distance;
                        }
                    }
                    original_distance =
                        self.project(*self.zp1().points.get(point)?, *self.zp0().points.get(rp0)?);
                    self.move_point(self.zp1, point, distance.wrapping_sub(original_distance))?;
                    rp1 = rp0;
                    rp2 = point;
                    if (opcode & 16) != 0 {
                        rp0 = point;
                    }
                }
                OP_MIRP00000..=OP_MIRP11111 => {
                    let point = *self.stack.get(a0)? as usize;
                    let cvt_entry = (*self.stack.get(a1)? + 1) as usize;
                    let mut cvt_distance = if cvt_entry == 0 {
                        0
                    } else {
                        *self.cvt.get(cvt_entry - 1)?
                    };
                    let cutin = state.gs.single_width_cutin;
                    let value = state.gs.single_width;
                    let mut delta = (cvt_distance - value).abs();
                    if delta < cutin {
                        cvt_distance = if cvt_distance >= 0 { value } else { -value };
                    }
                    if self.zp1 == 0 {
                        let fv = self.fv;
                        let p = {
                            let p2 = *self.zp0().original.get(rp0)?;
                            let p1 = self.zp1_mut().original.get_mut(point)?;
                            p1.x = p2.x + mul(cvt_distance, fv.x as i32);
                            p1.y = p2.y + mul(cvt_distance, fv.y as i32);
                            *p1
                        };
                        *self.zp1_mut().points.get_mut(point)? = p;
                    }
                    let original_distance = self.dual_project(
                        *self.zp1().original.get(point)?,
                        *self.zp0().original.get(rp0)?,
                    );
                    let current_distance =
                        self.project(*self.zp1().points.get(point)?, *self.zp0().points.get(rp0)?);
                    if state.gs.auto_flip && (original_distance ^ cvt_distance) < 0 {
                        cvt_distance = -cvt_distance;
                    }
                    let mut distance = if (opcode & 4) != 0 {
                        if self.zp0 == self.zp1 {
                            delta = (cvt_distance - original_distance).abs();
                            if delta > state.gs.control_value_cutin {
                                cvt_distance = original_distance;
                            }
                        }
                        self.round(cvt_distance)
                    } else {
                        cvt_distance
                    };
                    let min_distance = state.gs.min_distance;
                    if (opcode & 8) != 0 {
                        if original_distance >= 0 {
                            if distance < min_distance {
                                distance = min_distance
                            };
                        } else if distance > -min_distance {
                            distance = -min_distance
                        }
                    }
                    self.move_point(self.zp1, point, distance.wrapping_sub(current_distance))?;
                    rp1 = rp0;
                    if (opcode & 16) != 0 {
                        rp0 = point;
                    }
                    rp2 = point;
                }
                _ => {
                    let axis_count = self.axis_count as usize;
                    if axis_count != 0 && opcode == OP_GETVAR {
                        if stack_top + axis_count < self.stack.len() {
                            if axis_count == self.coords.len() {
                                for (sp, coord) in self
                                    .stack
                                    .get_mut(a0..a0 + axis_count)?
                                    .iter_mut()
                                    .zip(self.coords)
                                {
                                    *sp = *coord as i32;
                                }
                            } else {
                                for sp in self.stack.get_mut(a0..a0 + axis_count)? {
                                    *sp = 0;
                                }
                            }
                            new_top = stack_top + axis_count;
                        } else {
                            return None;
                        }
                    } else if axis_count != 0 && opcode == 0x92 {
                        *self.stack.get_mut(a0)? = 17;
                    } else {
                        let mut index = !0;
                        for i in 0..self.idefs.len() {
                            let idef = &self.idefs[i];
                            if idef.active && idef.opcode == opcode as u16 {
                                index = i;
                                break;
                            }
                        }
                        if index != !0 && callstack_top < callstack_len {
                            let def = self.idefs[index];
                            let rec = CallRecord {
                                caller_program: program,
                                caller_ip: pc + 1,
                                current_count: count as u32,
                                definition: def,
                            };
                            callstack[callstack_top] = rec;
                            callstack_top += 1;
                            program = def.program;
                            code = *programs.get(program as usize)?;
                            next_pc = def.offset as usize;
                        } else {
                            return None;
                        }
                    }
                }
            }

            if TRACE {
                // if trpt < self.glyph.points.len() {
                //     println!(
                //         ">>>>>> {}, {}",
                //         self.glyph.points[trpt].x, self.glyph.points[trpt].y
                //     );
                // }
            }

            count += 1;
            stack_top = new_top;
            pc = next_pc;
            if pc >= code.len() {
                if callstack_top > 0 {
                    return None;
                }
                break;
            }
        }
        Some(count)
    }
}

#[derive(Clone, Copy)]
struct GraphicsState {
    auto_flip: bool,
    control_value_cutin: i32,
    delta_base: u16,
    delta_shift: u16,
    instruct_control: u8,
    min_distance: i32,
    scan_control: bool,
    scan_type: i32,
    single_width_cutin: i32,
    single_width: i32,
}

const DEFAULT_GRAPHICS_STATE: GraphicsState = GraphicsState {
    auto_flip: true,
    control_value_cutin: 68,
    delta_base: 9,
    delta_shift: 3,
    instruct_control: 0,
    min_distance: 64,
    scan_control: false,
    scan_type: 0,
    single_width_cutin: 0,
    single_width: 0,
};

use math::*;

mod math {
    pub fn floor(x: i32) -> i32 {
        x & !63
    }

    pub fn ceil(x: i32) -> i32 {
        floor(x + 63)
    }

    pub fn round(x: i32) -> i32 {
        floor(x + 32)
    }

    pub fn floor_pad(x: i32, n: i32) -> i32 {
        x & !(n - 1)
    }

    pub fn round_pad(x: i32, n: i32) -> i32 {
        floor_pad(x + n / 2, n)
    }

    #[inline(always)]
    pub fn mul(a: i32, b: i32) -> i32 {
        let ab = a as i64 * b as i64;
        ((ab + 0x8000 - if ab < 0 { 1 } else { 0 }) >> 16) as i32
    }

    pub fn div(mut a: i32, mut b: i32) -> i32 {
        let mut s = 1;
        if a < 0 {
            a = -a;
            s = -1;
        }
        if b < 0 {
            b = -b;
            s = -s;
        }
        let q = if b == 0 {
            0x7FFFFFFF
        } else {
            ((((a as u64) << 16) + ((b as u64) >> 1)) / (b as u64)) as u32
        };
        if s < 0 {
            -(q as i32)
        } else {
            q as i32
        }
    }

    pub fn muldiv(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut s = 1;
        if a < 0 {
            a = -a;
            s = -1;
        }
        if b < 0 {
            b = -b;
            s = -s;
        }
        if c < 0 {
            c = -c;
            s = -s;
        }
        let d = if c > 0 {
            ((a as i64) * (b as i64) + ((c as i64) >> 1)) / c as i64
        } else {
            0x7FFFFFFF
        };
        if s < 0 {
            -(d as i32)
        } else {
            d as i32
        }
    }

    pub fn muldiv_no_round(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut s = 1;
        if a < 0 {
            a = -a;
            s = -1;
        }
        if b < 0 {
            b = -b;
            s = -s;
        }
        if c < 0 {
            c = -c;
            s = -s;
        }
        let d = if c > 0 {
            ((a as i64) * (b as i64)) / c as i64
        } else {
            0x7FFFFFFF
        };
        if s < 0 {
            -(d as i32)
        } else {
            d as i32
        }
    }

    pub fn mul14(a: i32, b: i32) -> i32 {
        let mut v = a as i64 * b as i64;
        v += 0x2000 + (v >> 63);
        (v >> 14) as i32
    }

    pub fn dot14(ax: i32, ay: i32, bx: i32, by: i32) -> i32 {
        let mut v1 = ax as i64 * bx as i64;
        let v2 = ay as i64 * by as i64;
        v1 += v2;
        v1 += 0x2000 + (v1 >> 63);
        (v1 >> 14) as i32
    }
}

use ops::*;

#[allow(dead_code)]
mod ops {
    pub const OPCODE_LEN: [i8; 256] = [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, -1, -2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 4, 5,
        6, 7, 8, 9, 3, 5, 7, 9, 11, 13, 15, 17, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];

    #[rustfmt::skip]
    #[allow(clippy::eq_op, clippy::identity_op)]
    pub const OPCODE_POP_PUSH: [u8; 256] = [
        (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0,
        (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0,
        (0 << 4) | 2, (0 << 4) | 2, (0 << 4) | 0, (5 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0,
        (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0,
        (0 << 4) | 0, (0 << 4) | 0, (1 << 4) | 0, (0 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0,
        (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 2, (1 << 4) | 0, (0 << 4) | 0, (2 << 4) | 2,
        (0 << 4) | 1, (1 << 4) | 1, (1 << 4) | 0, (2 << 4) | 0, (0 << 4) | 0, (1 << 4) | 0,
        (2 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (0 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0,
        (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0,
        (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (0 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0,
        (0 << 4) | 0, (0 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0,
        (2 << 4) | 0, (1 << 4) | 1, (2 << 4) | 0, (1 << 4) | 1, (1 << 4) | 1, (1 << 4) | 1,
        (2 << 4) | 0, (2 << 4) | 1, (2 << 4) | 1, (0 << 4) | 1, (0 << 4) | 1, (0 << 4) | 0,
        (0 << 4) | 0, (1 << 4) | 0, (2 << 4) | 1, (2 << 4) | 1, (2 << 4) | 1, (2 << 4) | 1,
        (2 << 4) | 1, (2 << 4) | 1, (1 << 4) | 1, (1 << 4) | 1, (1 << 4) | 0, (0 << 4) | 0,
        (2 << 4) | 1, (2 << 4) | 1, (1 << 4) | 1, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0,
        (2 << 4) | 1, (2 << 4) | 1, (2 << 4) | 1, (2 << 4) | 1, (1 << 4) | 1, (1 << 4) | 1,
        (1 << 4) | 1, (1 << 4) | 1, (1 << 4) | 1, (1 << 4) | 1, (1 << 4) | 1, (1 << 4) | 1,
        (1 << 4) | 1, (1 << 4) | 1, (1 << 4) | 1, (1 << 4) | 1, (2 << 4) | 0, (1 << 4) | 0,
        (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0,
        (2 << 4) | 0, (2 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0,
        (1 << 4) | 0, (1 << 4) | 0, (0 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (0 << 4) | 0,
        (0 << 4) | 0, (1 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (1 << 4) | 1, (1 << 4) | 0,
        (3 << 4) | 3, (2 << 4) | 1, (2 << 4) | 1, (1 << 4) | 0, (2 << 4) | 0, (0 << 4) | 0,
        (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 1, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0,
        (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0,
        (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0,
        (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0,
        (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 0,
        (0 << 4) | 0, (0 << 4) | 0, (0 << 4) | 1, (0 << 4) | 2, (0 << 4) | 3, (0 << 4) | 4,
        (0 << 4) | 5, (0 << 4) | 6, (0 << 4) | 7, (0 << 4) | 8, (0 << 4) | 1, (0 << 4) | 2,
        (0 << 4) | 3, (0 << 4) | 4, (0 << 4) | 5, (0 << 4) | 6, (0 << 4) | 7, (0 << 4) | 8,
        (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0,
        (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0,
        (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0,
        (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0,
        (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0, (1 << 4) | 0,
        (1 << 4) | 0, (1 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0,
        (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0,
        (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0,
        (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0,
        (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0,
        (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0, (2 << 4) | 0,
    ];

    #[rustfmt::skip]
    pub const OPCODE_NAME: [&'static str; 256] = [
        "SVTCA0", "SVTCA1", "SPVTCA0", "SPVTCA1", "SFVTCA0", "SFVTCA1", "SPVTL0", "SPVTL1", "SFVTL0",
        "SFVTL1", "SPVFS", "SFVFS", "GPV", "GFV", "SFVTPV", "ISECT", "SRP0", "SRP1", "SRP2", "SZP0",
        "SZP1", "SZP2", "SZPS", "SLOOP", "RTG", "RTHG", "SMD", "ELSE", "JMPR", "SCVTCI", "SSWCI",
        "SSW", "DUP", "POP", "CLEAR", "SWAP", "DEPTH", "CINDEX", "MINDEX", "ALIGNPTS", "OP28", "UTP",
        "LOOPCALL", "CALL", "FDEF", "ENDF", "MDAP0", "MDAP1", "IUP0", "IUP1", "SHP0", "SHP1", "SHC0",
        "SHC1", "SHZ0", "SHZ1", "SHPIX", "IP", "MSIRP0", "MSIRP1", "ALIGNRP", "RTDG", "MIAP0", "MIAP1",
        "NPUSHB", "NPUSHW", "WS", "RS", "WCVTP", "RCVT", "GC0", "GC1", "SCFS", "MD0", "MD1", "MPPEM",
        "MPS", "FLIPON", "FLIPOFF", "DEBUG", "LT", "LTEQ", "GT", "GTEQ", "EQ", "NEQ", "ODD", "EVEN",
        "IF", "EIF", "AND", "OR", "NOT", "DELTAP1", "SDB", "SDS", "ADD", "SUB", "DIV", "MUL", "ABS",
        "NEG", "FLOOR", "CEILING", "ROUND00", "ROUND01", "ROUND10", "ROUND11", "NROUND00", "NROUND01",
        "NROUND10", "NROUND11", "WCVTF", "DELTAP2", "DELTAP3", "DELTAC1", "DELTAC2", "DELTAC3",
        "SROUND", "S45ROUND", "JROT", "JROF", "ROFF", "OP7B", "RUTG", "RDTG", "SANGW", "AA", "FLIPPT",
        "FLIPRGON", "FLIPRGOFF", "OP83", "OP84", "SCANCTRL", "SDPVTL0", "SDPVTL1", "GETINFO", "IDEF",
        "ROLL", "MAX", "MIN", "SCANTYPE", "INSTCTRL", "OP8F", "OP90", "OP91", "OP92", "OP93", "OP94",
        "OP95", "OP96", "OP97", "OP98", "OP99", "OP9A", "OP9B", "OP9C", "OP9D", "OP9E", "OP9F", "OPA0",
        "OPA1", "OPA2", "OPA3", "OPA4", "OPA5", "OPA6", "OPA7", "OPA8", "OPA9", "OPAA", "OPAB", "OPAC",
        "OPAD", "OPAE", "OPAF", "PUSHB000", "PUSHB001", "PUSHB010", "PUSHB011", "PUSHB100", "PUSHB101",
        "PUSHB110", "PUSHB111", "PUSHW000", "PUSHW001", "PUSHW010", "PUSHW011", "PUSHW100", "PUSHW101",
        "PUSHW110", "PUSHW111", "MDRP00000", "MDRP00001", "MDRP00010", "MDRP00011", "MDRP00100",
        "MDRP00101", "MDRP00110", "MDRP00111", "MDRP01000", "MDRP01001", "MDRP01010", "MDRP01011",
        "MDRP01100", "MDRP01101", "MDRP01110", "MDRP01111", "MDRP10000", "MDRP10001", "MDRP10010",
        "MDRP10011", "MDRP10100", "MDRP10101", "MDRP10110", "MDRP10111", "MDRP11000", "MDRP11001",
        "MDRP11010", "MDRP11011", "MDRP11100", "MDRP11101", "MDRP11110", "MDRP11111", "MIRP00000",
        "MIRP00001", "MIRP00010", "MIRP00011", "MIRP00100", "MIRP00101", "MIRP00110", "MIRP00111",
        "MIRP01000", "MIRP01001", "MIRP01010", "MIRP01011", "MIRP01100", "MIRP01101", "MIRP01110",
        "MIRP01111", "MIRP10000", "MIRP10001", "MIRP10010", "MIRP10011", "MIRP10100", "MIRP10101",
        "MIRP10110", "MIRP10111", "MIRP11000", "MIRP11001", "MIRP11010", "MIRP11011", "MIRP11100",
        "MIRP11101", "MIRP11110", "MIRP11111",
    ];

    pub const OP_SVTCA0: u8 = 0x00;
    pub const OP_SFVTCA1: u8 = 0x05;
    pub const OP_SPVTL0: u8 = 0x06;
    pub const OP_SPVTL1: u8 = 0x07;
    pub const OP_SFVTL1: u8 = 0x09;
    pub const OP_SPVFS: u8 = 0x0A;
    pub const OP_SFVFS: u8 = 0x0B;
    pub const OP_GPV: u8 = 0x0C;
    pub const OP_GFV: u8 = 0x0D;
    pub const OP_SFVTPV: u8 = 0x0E;
    pub const OP_ISECT: u8 = 0x0F;
    pub const OP_SRP0: u8 = 0x10;
    pub const OP_SRP1: u8 = 0x11;
    pub const OP_SRP2: u8 = 0x12;
    pub const OP_SZP0: u8 = 0x13;
    pub const OP_SZP1: u8 = 0x14;
    pub const OP_SZP2: u8 = 0x15;
    pub const OP_SZPS: u8 = 0x16;
    pub const OP_SLOOP: u8 = 0x17;
    pub const OP_RTG: u8 = 0x18;
    pub const OP_RTHG: u8 = 0x19;
    pub const OP_SMD: u8 = 0x1A;
    pub const OP_ELSE: u8 = 0x1B;
    pub const OP_JMPR: u8 = 0x1C;
    pub const OP_SCVTCI: u8 = 0x1D;
    pub const OP_SSWCI: u8 = 0x1E;
    pub const OP_SSW: u8 = 0x1F;
    pub const OP_DUP: u8 = 0x20;
    pub const OP_POP: u8 = 0x21;
    pub const OP_CLEAR: u8 = 0x22;
    pub const OP_SWAP: u8 = 0x23;
    pub const OP_DEPTH: u8 = 0x24;
    pub const OP_CINDEX: u8 = 0x25;
    pub const OP_MINDEX: u8 = 0x26;
    pub const OP_ALIGNPTS: u8 = 0x27;
    pub const OP_UTP: u8 = 0x29;
    pub const OP_LOOPCALL: u8 = 0x2A;
    pub const OP_CALL: u8 = 0x2B;
    pub const OP_FDEF: u8 = 0x2C;
    pub const OP_ENDF: u8 = 0x2D;
    pub const OP_MDAP0: u8 = 0x2E;
    pub const OP_MDAP1: u8 = 0x2F;
    pub const OP_IUP0: u8 = 0x30;
    pub const OP_IUP1: u8 = 0x31;
    pub const OP_SHP0: u8 = 0x32;
    pub const OP_SHP1: u8 = 0x33;
    pub const OP_SHC0: u8 = 0x34;
    pub const OP_SHC1: u8 = 0x35;
    pub const OP_SHZ0: u8 = 0x36;
    pub const OP_SHZ1: u8 = 0x37;
    pub const OP_SHPIX: u8 = 0x38;
    pub const OP_IP: u8 = 0x39;
    pub const OP_MSIRP0: u8 = 0x3A;
    pub const OP_MSIRP1: u8 = 0x3B;
    pub const OP_ALIGNRP: u8 = 0x3C;
    pub const OP_RTDG: u8 = 0x3D;
    pub const OP_MIAP0: u8 = 0x3E;
    pub const OP_MIAP1: u8 = 0x3F;
    pub const OP_NPUSHB: u8 = 0x40;
    pub const OP_NPUSHW: u8 = 0x41;
    pub const OP_WS: u8 = 0x42;
    pub const OP_RS: u8 = 0x43;
    pub const OP_WCVTP: u8 = 0x44;
    pub const OP_RCVT: u8 = 0x45;
    pub const OP_GC0: u8 = 0x46;
    pub const OP_GC1: u8 = 0x47;
    pub const OP_SCFS: u8 = 0x48;
    pub const OP_MD0: u8 = 0x49;
    pub const OP_MD1: u8 = 0x4A;
    pub const OP_MPPEM: u8 = 0x4B;
    pub const OP_MPS: u8 = 0x4C;
    pub const OP_FLIPON: u8 = 0x4D;
    pub const OP_FLIPOFF: u8 = 0x4E;
    pub const OP_DEBUG: u8 = 0x4F;
    pub const OP_LT: u8 = 0x50;
    pub const OP_LTEQ: u8 = 0x51;
    pub const OP_GT: u8 = 0x52;
    pub const OP_GTEQ: u8 = 0x53;
    pub const OP_EQ: u8 = 0x54;
    pub const OP_NEQ: u8 = 0x55;
    pub const OP_ODD: u8 = 0x56;
    pub const OP_EVEN: u8 = 0x57;
    pub const OP_IF: u8 = 0x58;
    pub const OP_EIF: u8 = 0x59;
    pub const OP_AND: u8 = 0x5A;
    pub const OP_OR: u8 = 0x5B;
    pub const OP_NOT: u8 = 0x5C;
    pub const OP_DELTAP1: u8 = 0x5D;
    pub const OP_SDB: u8 = 0x5E;
    pub const OP_SDS: u8 = 0x5F;
    pub const OP_ADD: u8 = 0x60;
    pub const OP_SUB: u8 = 0x61;
    pub const OP_DIV: u8 = 0x62;
    pub const OP_MUL: u8 = 0x63;
    pub const OP_ABS: u8 = 0x64;
    pub const OP_NEG: u8 = 0x65;
    pub const OP_FLOOR: u8 = 0x66;
    pub const OP_CEILING: u8 = 0x67;
    pub const OP_ROUND00: u8 = 0x68;
    pub const OP_ROUND11: u8 = 0x6B;
    pub const OP_NROUND00: u8 = 0x6C;
    pub const OP_NROUND11: u8 = 0x6F;
    pub const OP_WCVTF: u8 = 0x70;
    pub const OP_DELTAP2: u8 = 0x71;
    pub const OP_DELTAP3: u8 = 0x72;
    pub const OP_DELTAC1: u8 = 0x73;
    pub const OP_DELTAC2: u8 = 0x74;
    pub const OP_DELTAC3: u8 = 0x75;
    pub const OP_SROUND: u8 = 0x76;
    pub const OP_S45ROUND: u8 = 0x77;
    pub const OP_JROT: u8 = 0x78;
    pub const OP_JROF: u8 = 0x79;
    pub const OP_ROFF: u8 = 0x7A;
    pub const OP_RUTG: u8 = 0x7C;
    pub const OP_RDTG: u8 = 0x7D;
    pub const OP_SANGW: u8 = 0x7E;
    pub const OP_AA: u8 = 0x7F;
    pub const OP_FLIPPT: u8 = 0x80;
    pub const OP_FLIPRGON: u8 = 0x81;
    pub const OP_FLIPRGOFF: u8 = 0x82;
    pub const OP_SCANCTRL: u8 = 0x85;
    pub const OP_SDPVTL0: u8 = 0x86;
    pub const OP_SDPVTL1: u8 = 0x87;
    pub const OP_GETINFO: u8 = 0x88;
    pub const OP_IDEF: u8 = 0x89;
    pub const OP_ROLL: u8 = 0x8A;
    pub const OP_MAX: u8 = 0x8B;
    pub const OP_MIN: u8 = 0x8C;
    pub const OP_SCANTYPE: u8 = 0x8D;
    pub const OP_INSTCTRL: u8 = 0x8E;
    pub const OP_GETVAR: u8 = 0x91;
    pub const OP_PUSHB000: u8 = 0xB0;
    pub const OP_PUSHB111: u8 = 0xB7;
    pub const OP_PUSHW000: u8 = 0xB8;
    pub const OP_PUSHW111: u8 = 0xBF;
    pub const OP_MDRP00000: u8 = 0xC0;
    pub const OP_MDRP11111: u8 = 0xDF;
    pub const OP_MIRP00000: u8 = 0xE0;
    pub const OP_MIRP11111: u8 = 0xFF;

    // Rounding modes
    pub const ROUND_TO_HALF_GRID: u8 = 0;
    pub const ROUND_TO_GRID: u8 = 1;
    pub const ROUND_TO_DOUBLE_GRID: u8 = 2;
    pub const ROUND_DOWN_TO_GRID: u8 = 3;
    pub const ROUND_UP_TO_GRID: u8 = 4;
    pub const ROUND_OFF: u8 = 5;
    pub const ROUND_SUPER: u8 = 6;
    pub const ROUND_SUPER45: u8 = 7;

    // Tag bits
    pub const TOUCH_X: u8 = 0x08;
    pub const TOUCH_Y: u8 = 0x10;
}
