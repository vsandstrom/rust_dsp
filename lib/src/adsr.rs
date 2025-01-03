use core::f32;

#[derive(Debug, Default)]

pub enum Reset {
  Hard, 
  #[default] Soft
}

#[derive(Debug, Default)]
enum EnvStage {
  #[default] Atk,
  Dec,
  Sus,
  Rel
}

#[derive(Debug)]
pub struct ADSREnvelope {
  atk_value: f32,
  atk_duration: f32,
  atk_curve: f32,

  dec_duration: f32,
  dec_curve: f32,

  sus_value: f32,

  rel_duration: f32,
  rel_curve: f32,

  stage: EnvStage,
  start: f32,
  prev: f32,
  next: f32,
  playing: bool,
  reset: Reset,
  count: usize,
  sr: f32
}

impl ADSREnvelope {
  pub fn new(sr: f32) -> Self {
    Self {
      atk_value: 1.0,
      atk_duration: 1.0,
      atk_curve: 0.2,
      dec_duration: 0.5,
      dec_curve: 0.8,
      sus_value: 0.5,
      rel_duration: 2.0,
      rel_curve: 0.8,
      stage: EnvStage::Atk,
      start: f32::EPSILON,
      prev: f32::EPSILON,
      next: f32::EPSILON,
      playing: false,
      reset: Reset::Soft,
      count: 0,
      sr
    }
  }



  pub fn play(&mut self, trig: bool, sustain: bool) -> f32 {
    debug_assert!(self.sr > f32::EPSILON, "forgotten to set the samplerate?");
    if trig { self.handle_trig(); }
    if !self.playing { return 0.0; }

    let env = match self.stage {
      EnvStage::Atk => {
        self.count += 1;
        self.process(self.start, self.atk_value, self.atk_duration, self.atk_curve, self.count)
      },
      EnvStage::Dec => {
        self.count += 1;
        self.process(self.prev, self.sus_value, self.dec_duration, self.dec_curve, self.count)
      },
      EnvStage::Sus => {
        self.prev
      },
      EnvStage::Rel => {
        self.count += 1;
        self.process(self.prev, 0.0001f32, self.rel_duration, self.rel_curve, self.count)
      }
    };

    match self.stage {
      EnvStage::Atk => {
        if self.count >= (self.atk_duration * self.sr) as usize { 
          self.stage = EnvStage::Dec; 
          self.count = 0;
          self.prev = env;
        }
      },
      EnvStage::Dec => {
        if self.count >= (self.dec_duration * self.sr) as usize { 
          self.stage = EnvStage::Sus; 
          self.count = 0; 
          self.prev = env;
        }
      },
      EnvStage::Sus => {
        if !sustain { 
          self.stage = EnvStage::Rel;
          self.count = 0;
          self.prev = env;
        }
      },
      EnvStage::Rel => {
        if self.count >= (self.rel_duration * self.sr) as usize { 
          self.stage = EnvStage::Atk; 
          self.playing = false;
          self.prev = env;
        }
      },
    }
    self.next = env;
    env
  }

  fn process(&self, start: f32, end: f32, dur: f32, curve: f32, count: usize) -> f32 {
    let t = count as f32 / (dur * self.sr);
    if start > end { start - f32::powf(t, curve) * (start - end) }
    else           { start + f32::powf(t, curve) * (end - start) }
  }

  fn handle_trig(&mut self) {
    match self.reset {
      Reset::Hard => { self.start = 0f32; },
      Reset::Soft => { self.start = self.next; }
    }
    self.playing = true;
    self.count = 0;
    self.stage = EnvStage::Atk;
  }

  pub fn set_attack_val (&mut self,    atk_value: f32) { self.atk_value    = atk_value; }
  pub fn set_attack_dur (&mut self, atk_duration: f32) { self.atk_duration = atk_duration; }
  pub fn set_attack_cur (&mut self,    atk_curve: f32) { self.atk_curve    = atk_curve; }
  pub fn set_decay_dur  (&mut self, dec_duration: f32) { self.dec_duration = dec_duration; }
  pub fn set_decay_cur  (&mut self,    dec_curve: f32) { self.dec_curve    = dec_curve; }
  pub fn set_sustain_val(&mut self,    sus_value: f32) { self.sus_value    = sus_value; }
  pub fn set_release_dur(&mut self, rel_duration: f32) { self.rel_duration = rel_duration; }
  pub fn set_release_cur(&mut self,    rel_curve: f32) { self.rel_curve    = rel_curve; }
  pub fn set_reset_type (&mut self, reset: Reset)      { self.reset        = reset; }
}

