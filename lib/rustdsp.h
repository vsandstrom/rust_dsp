#ifndef RUST_DSP_H
#define RUST_DSP_H

#pragma once

#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace rustdsp {

enum class Reset {
  Hard,
  Soft,
};

/// /* Underlying Structure */
/// ```ignore
/// pub struct Granulator {
///   buffer: Vec<f32>,
///   buf_size: usize,
///   envelope: Vec<f32>,
///   env_size: usize,
///   rec_pos: usize,
///   pub recording: bool,
///   next_grain: usize,
///   grains: Vec<Grain>,
///   samplerate: f32,
///   sr_recip: f32,
/// }
/// ```
struct GranulatorOpaque {

};

struct WavetableOpaque {

};

struct BiquadOpaque {

};

struct BiquadCoeffs {
  float a1;
  float a2;
  float b0;
  float b1;
  float b2;
};

struct Biquad4Opaque {

};

struct Biquad8Opaque {

};

/// struct Delay {
///   buffer: Vec<f32>,
///   delay: f32,
///   position: usize,
/// }
struct DelayOpaque {

};

struct EnvelopeOpaque {

};

/// ```ignore
/// Underlying structure:
/// #[derive(Debug)]
/// struct ADSREnvelope {
///   atk_value: f32,
///   atk_duration: f32,
///   atk_curve: f32,
///
///   dec_duration: f32,
///   dec_curve: f32,
///
///   sus_value: f32,
///
///   rel_duration: f32,
///   rel_curve: f32,
///
///   stage: EnvStage,
///   start: f32,
///   prev: f32,
///   next: f32,
///   playing: bool,
///   reset: Reset,
///   count: usize,
///   sr: f32
/// }
/// ```
struct ADSREnvelopeOpaque {

};

extern "C" {

float clamp_signal(float signal, float bottom, float top);

/// Map a signal of range m -> n into new range, x -> y
float signal_map(float *signal, float in_min, float in_max, float out_min, float out_max);

float signal_dcblock(float signal, float xm1, float ym1);

/// Convenience for normalizing a signal to be only positive.
float signal_unipolar(float sample);

/// calculates panning weights for stereo equal power panning.
/// returns a pointer to an array of len 2, [left, right]
const float *signal_pan_exp2(float pan);

/// calculates panning weights for stereo linear panning.
/// returns a pointer to an array of len 2, [left, right]
const float *signal_pan_lin2(float pan);

/// Same as map, but for entire buffers. Suitable for normalizing Wavetable buffers.
/// # Safety
/// Reads a raw pointer into a rust slice.
const float *buffer_range(float *values,
                          size_t len,
                          float in_min,
                          float in_max,
                          float out_min,
                          float out_max);

/// Calculates the sum of all values in array
/// # Safety
/// Reads a raw pointer into a rust slice.
float buffer_sum(const float *values, size_t len);

/// Normalizes contents of vec, sum of contents == 1.0
/// # Safety
/// Reads a raw pointer into a rust slice.
void buffer_normalize(float *values, size_t len);

/// Scales the contents of a Vec to be between outmin -> outmax
/// # Safety
/// Reads a raw pointer into a rust slice.
/// (should mutate contents of array in place)
void buffer_scale(float *values, size_t len, float outmin, float outmax);

size_t math_next_pow2(size_t size);

bool math_is_pow2(size_t size);

float math_midi_to_freq(uint8_t midi, float tuning);

uint8_t math_freq_to_midi(float freq, float tuning);

float math_midi_to_rate(uint8_t midi);

float hz_to_radian(float hz, float samplerate);

float math_db_to_volume(float db);

float math_volume_to_db(float volume);

float math_samples_to_wavelength(size_t samples, float samplerate);

size_t wavelength_to_samples(float wavelength, float samplerate);

void shape_complex_sine(float *table,
                        size_t size,
                        const float *amps,
                        size_t asize,
                        const float *phases,
                        size_t psize);

void shape_sine(float *table, size_t size);

void shape_hanning(float *table, size_t size);

void shape_square(float *table, size_t size);

void shape_triangle(float *table, size_t size);

void shape_reverse_sawtooth(float *table, size_t size);

void shape_sawtooth(float *table, size_t size);

/// Constructor
GranulatorOpaque *granulator_new(float samplerate, size_t num_grains, size_t buf_size);

/// Destructor
void granulator_delete(GranulatorOpaque *granulator);

/// Trigger new grain
bool granulator_trigger(GranulatorOpaque *granulator,
                        float position,
                        float duration,
                        float rate,
                        float jitter);

/// Play with linear buffer interpolation
float granulator_play_linear(GranulatorOpaque *granulator);

/// Play with cubic buffer interpolation
float granulator_play_cubic(GranulatorOpaque *granulator);

/// Record into buffer
bool granulator_record(GranulatorOpaque *granulator, float sample);

/// Underlying structure:
/// ```ignore
/// pub struct Wavetable {
///   position: f32,
///   samplerate: f32,
///   sr_recip: f32,
/// }
/// ```
/// Constructor
WavetableOpaque *wavetable_new();

/// Destructor
void wavetable_delete(WavetableOpaque *wavetable);

void wavetable_set_samplerate(WavetableOpaque *wavetable, float samplerate);

float wavetable_play_floor(WavetableOpaque *wavetable,
                           const float *table,
                           size_t table_length,
                           float frequency,
                           float phase);

float wavetable_play_linear(WavetableOpaque *wavetable,
                            const float *table,
                            size_t table_length,
                            float frequency,
                            float phase);

float wavetable_play_cubic(WavetableOpaque *wavetable,
                           const float *table,
                           size_t table_length,
                           float frequency,
                           float phase);

/// Constructor
BiquadOpaque *biquad_new();

/// Destructor
void biquad_delete(BiquadOpaque *biquad);

float biquad_process(BiquadOpaque *biquad, float sample);

void biquad_update(BiquadOpaque *biquad, BiquadCoeffs coeffs);

void biquad_calc_lpf(BiquadOpaque *biquad, float w, float q);

void biquad_calc_bpf(BiquadOpaque *biquad, float w, float q);

void biquad_calc_hpf(BiquadOpaque *biquad, float w, float q);

void biquad_calc_notch(BiquadOpaque *biquad, float w, float q);

void biquad_calc_peq(BiquadOpaque *biquad, float w, float q, float gain);

/// Constructor
Biquad4Opaque *biquad4_new();

/// Destructor
void biquad4_delete(Biquad4Opaque *biquad4);

float biquad4_process(Biquad4Opaque *biquad4, float sample);

void biquad4_update(Biquad4Opaque *biquad4, BiquadCoeffs coeffs);

void biquad4_calc_lpf(Biquad4Opaque *biquad4, float w, float q);

void biquad4_calc_bpf(Biquad4Opaque *biquad4, float w, float q);

void biquad4_calc_hpf(Biquad4Opaque *biquad4, float w, float q);

void biquad4_calc_notch(Biquad4Opaque *biquad4, float w, float q);

void biquad4_calc_peq(Biquad4Opaque *biquad4, float w, float q, float gain);

/// Constructor
Biquad8Opaque *biquad8_new();

/// Destructor
void biquad8_delete(Biquad8Opaque *biquad8);

float biquad8_process(Biquad8Opaque *biquad8, float sample);

void biquad8_update(Biquad8Opaque *biquad8, BiquadCoeffs coeffs);

void biquad8_calc_lpf(Biquad8Opaque *biquad8, float w, float q);

void biquad8_calc_bpf(Biquad8Opaque *biquad8, float w, float q);

void biquad8_calc_hpf(Biquad8Opaque *biquad8, float w, float q);

void biquad8_calc_notch(Biquad8Opaque *biquad8, float w, float q);

void biquad8_calc_peq(Biquad8Opaque *biquad8, float w, float q, float gain);

BiquadCoeffs calc_lpf(float w, float q);

BiquadCoeffs calc_bpf(float w, float q);

BiquadCoeffs calc_hpf(float w, float q);

BiquadCoeffs calc_notch(float w, float q);

BiquadCoeffs calc_peq(float w, float q, float gain);

/// Constructor
DelayOpaque *delay_new(size_t length);

/// Destructor
void delay_delete(DelayOpaque *delay);

float delay_play_linear(DelayOpaque *delay, float input, float seconds, float feedback);

float delay_play_cubic(DelayOpaque *delay, float input, float seconds, float feedback);

EnvelopeOpaque *envelope_new(const float *value,
                             size_t v_len,
                             const float *duration,
                             size_t d_len,
                             const float *curve,
                             size_t c_len,
                             float samplerate);

void envelope_delete(EnvelopeOpaque *env);

void envelope_trig(EnvelopeOpaque *env);

float envelope_play(EnvelopeOpaque *env);

void envelope_set_reset_type(EnvelopeOpaque *env, Reset reset_type);

void envelope_loopable(EnvelopeOpaque *env, bool loopable);

/// Constructor
ADSREnvelopeOpaque *adsr_new(float samplerate);

/// Destructor
void adsr_delete(ADSREnvelopeOpaque *adsr);

void adsr_set_attack_val(ADSREnvelopeOpaque *adsr, float atk_value);

void adsr_set_attack_dur(ADSREnvelopeOpaque *adsr, float atk_duration);

void adsr_set_attack_cur(ADSREnvelopeOpaque *adsr, float atk_curve);

void adsr_set_decay_dur(ADSREnvelopeOpaque *adsr, float dec_duration);

void adsr_set_decay_cur(ADSREnvelopeOpaque *adsr, float dec_curve);

void adsr_set_sustain_val(ADSREnvelopeOpaque *adsr, float sus_value);

void adsr_set_release_dur(ADSREnvelopeOpaque *adsr, float rel_duration);

void adsr_set_release_cur(ADSREnvelopeOpaque *adsr, float rel_curve);

void adsr_set_reset_type(ADSREnvelopeOpaque *adsr, Reset reset);

float adsr_play(ADSREnvelopeOpaque *adsr, bool trig, bool sustain);

float fold_process(float input, float amount);

}  // extern "C"

}  // namespace rustdsp

#endif  // RUST_DSP_H
