use std::ops::{Deref, DerefMut};
use std::slice;

use ::{AltoError, AltoResult};
use sys;
use alc::*;
use al::*;
use ext;


/// Audio formats supported by OpenAL.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Format {
	Standard(StandardFormat),
	ExtALaw(ExtALawFormat),
	ExtBFormat(ExtBFormat),
	ExtDouble(ExtDoubleFormat),
	ExtFloat32(ExtFloat32Format),
	ExtIma4(ExtIma4Format),
	ExtMcFormats(ExtMcFormat),
	ExtMuLaw(ExtMuLawFormat),
	ExtMuLawBFormat(ExtMuLawBFormat),
	ExtMuLawMcFormats(ExtMuLawMcFormat),
	SoftMsadpcm(SoftMsadpcmFormat),
}


/// Standard formats defined in the base specification.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum StandardFormat {
	/// `AL_FORMAT_MONO8`
	MonoU8,
	/// `AL_FORMAT_MONO16`
	MonoI16,
	/// `AL_FORMAT_STEREO8`
	StereoU8,
	/// `AL_FORMAT_STEREO16`
	StereoI16,
}


/// Formats provided by `AL_EXT_ALAW`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtALawFormat {
	/// `AL_FORMAT_MONO_ALAW_EXT`
	Mono,
	/// `AL_FORMAT_STEREO_ALAW_EXT`
	Stereo,
}


/// Formats provided by `AL_EXT_BFORMAT`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtBFormat {
	/// `AL_FORMAT_BFORMAT2D_8`
	B2DU8,
	/// `AL_FORMAT_BFORMAT2D_16`
	B2DI16,
	/// `AL_FORMAT_BFORMAT2D_FLOAT32`
	B2DF32,
	/// `AL_FORMAT_BFORMAT3D_8`
	B3DU8,
	/// `AL_FORMAT_BFORMAT3D_16`
	B3DI16,
	/// `AL_FORMAT_BFORMAT3D_FLOAT32`
	B3DF32,
}


/// Formats provided by `AL_EXT_double`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtDoubleFormat {
	/// `AL_FORMAT_MONO_DOUBLE_EXT`
	Mono,
	/// `AL_FORMAT_STEREO_DOUBLE_EXT`
	Stereo,
}


/// Formats provided by `AL_EXT_float32`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtFloat32Format {
	/// `AL_FORMAT_MONO_FLOAT32`
	Mono,
	/// `AL_FORMAT_STEREO_FLOAT32`
	Stereo,
}


/// Formats provided by `AL_EXT_IMA4`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtIma4Format {
	/// `AL_FORMAT_MONO_IMA4`
	Mono,
	/// `AL_FORMAT_STEREO_IMA4`
	Stereo,
}


/// Formats provided by `AL_EXT_MCFORMATS`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtMcFormat {
	/// `AL_FORMAT_QUAD8`
	QuadU8,
	/// `AL_FORMAT_QUAD16`
	QuadI16,
	/// `AL_FORMAT_QUAD32`
	QuadF32,
	/// `AL_FORMAT_REAR8`
	RearU8,
	/// `AL_FORMAT_REAR16`
	RearI16,
	/// `AL_FORMAT_REAR32`
	RearF32,
	/// `AL_FORMAT_51CHN8`
	Mc51ChnU8,
	/// `AL_FORMAT_51CHN16`
	Mc51ChnI16,
	/// `AL_FORMAT_51CHN32`
	Mc51ChnF32,
	/// `AL_FORMAT_61CHN8`
	Mc61ChnU8,
	/// `AL_FORMAT_61CHN16`
	Mc61ChnI16,
	/// `AL_FORMAT_61CHN32`
	Mc61ChnF32,
	/// `AL_FORMAT_71CHN8`
	Mc71ChnU8,
	/// `AL_FORMAT_71CHN16`
	Mc71ChnI16,
	/// `AL_FORMAT_71CHN32`
	Mc71ChnF32,
}


/// Formats provided by `AL_EXT_MULAW`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtMuLawFormat {
	/// `AL_FORMAT_MONO_MULAW_EXT`
	Mono,
	/// `AL_FORMAT_STEREO_MULAW_EXT`
	Stereo,
}


/// Formats provided by `AL_EXT_MULAW_BFORMAT`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtMuLawBFormat {
	/// `AL_FORMAT_BFORMAT2D_MULAW`
	B2D,
	/// `AL_FORMAT_BFORMAT3D_MULAW`
	B3D,
}


/// Formats provided by `AL_EXT_MULAW_MCFORMATS`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExtMuLawMcFormat {
	/// `AL_FORMAT_MONO_MULAW`
	Mono,
	/// `AL_FORMAT_STEREO_MULAW`
	Stereo,
	/// `AL_FORMAT_QUAD_MULAW`
	Quad,
	/// `AL_FORMAT_REAR_MULAW`
	Rear,
	/// `AL_FORMAT_51CHN_MULAW`
	Mc51Chn,
	/// `AL_FORMAT_61CHN_MULAW`
	Mc61Chn,
	/// `AL_FORMAT_71CHN_MULAW`
	Mc71Chn,
}


/// Formats provided by `AL_SOFT_MSADPCM`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SoftMsadpcmFormat {
	/// `AL_FORMAT_MONO_MSADPCM_SOFT`
	Mono,
	/// `AL_FORMAT_STEREO_MSADPCM_SOFT`
	Stereo,
}


/// Implemented by structs that represent a frame of audio samples.
/// A sample frame is a grouping of audio samples from each channel
/// of an output format.
pub unsafe trait SampleFrame: Copy {
	/// Underlying sample type.
	type Sample: Copy;


	/// Length of the frame in samples.
	fn len() -> usize;
	/// The exact format described by this struct.
	fn format() -> Format;
}


/// Implemented for sample frames specified by the base standard.
pub unsafe trait StandardFrame: SampleFrame { }


pub trait AsBufferData<F: SampleFrame> {
	fn as_buffer_data(&self) -> &[F];
}


pub trait AsBufferDataMut<F: SampleFrame> {
	fn as_buffer_data_mut(&mut self) -> &mut [F];
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct ALawSample(pub u8);
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct MuLawSample(pub u8);


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct Mono<S: Copy> {
	pub center: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct Stereo<S: Copy> {
	pub left: S,
	pub right: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct McRear<S: Copy> {
	pub rear: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct McQuad<S: Copy> {
	pub front_left: S,
	pub front_right: S,
	pub back_left: S,
	pub back_right: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct Mc51Chn<S: Copy> {
	pub front_left: S,
	pub front_right: S,
	pub front_center: S,
	pub low_freq: S,
	pub back_left: S,
	pub back_right: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct Mc61Chn<S: Copy> {
	pub front_left: S,
	pub front_right: S,
	pub front_center: S,
	pub low_freq: S,
	pub back_left: S,
	pub back_right: S,
	pub back_center: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct Mc71Chn<S: Copy> {
	pub front_left: S,
	pub front_right: S,
	pub front_center: S,
	pub low_freq: S,
	pub back_left: S,
	pub back_right: S,
	pub side_left: S,
	pub side_right: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct BFormat2D<S: Copy> {
	pub w: S,
	pub x: S,
	pub y: S,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct BFormat3D<S: Copy> {
	pub w: S,
	pub x: S,
	pub y: S,
	pub z: S,
}


impl Format {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		match self {
			Format::Standard(f) => Ok(f.into_raw()),
			Format::ExtALaw(f) => f.into_raw(ctx),
			Format::ExtBFormat(f) => f.into_raw(ctx),
			Format::ExtDouble(f) => f.into_raw(ctx),
			Format::ExtFloat32(f) => f.into_raw(ctx),
			Format::ExtIma4(f) => f.into_raw(ctx),
			Format::ExtMcFormats(f) => f.into_raw(ctx),
			Format::ExtMuLaw(f) => f.into_raw(ctx),
			Format::ExtMuLawBFormat(f) => f.into_raw(ctx),
			Format::ExtMuLawMcFormats(f) => f.into_raw(ctx),
			Format::SoftMsadpcm(f) => f.into_raw(ctx),
		}
	}
}


impl StandardFormat {
	pub fn into_raw(self) -> sys::ALint {
		match self {
			StandardFormat::MonoU8 => sys::AL_FORMAT_MONO8,
			StandardFormat::MonoI16 => sys::AL_FORMAT_MONO16,
			StandardFormat::StereoU8 => sys::AL_FORMAT_STEREO8,
			StandardFormat::StereoI16 => sys::AL_FORMAT_STEREO16,
		}
	}
}


impl ExtALawFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtALawFormat::Mono => Ok(ctx.extensions().AL_EXT_ALAW()?.AL_FORMAT_MONO_ALAW_EXT?),
			ExtALawFormat::Stereo => Ok(ctx.extensions().AL_EXT_ALAW()?.AL_FORMAT_STEREO_ALAW_EXT?),
		})
	}
}


impl ExtBFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtBFormat::B2DU8 => Ok(ctx.extensions().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT2D_8?),
			ExtBFormat::B2DI16 => Ok(ctx.extensions().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT2D_16?),
			ExtBFormat::B2DF32 => Ok(ctx.extensions().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT2D_FLOAT32?),
			ExtBFormat::B3DU8 => Ok(ctx.extensions().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT3D_8?),
			ExtBFormat::B3DI16 => Ok(ctx.extensions().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT3D_16?),
			ExtBFormat::B3DF32 => Ok(ctx.extensions().AL_EXT_BFORMAT()?.AL_FORMAT_BFORMAT3D_FLOAT32?),
		})
	}
}


impl ExtDoubleFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtDoubleFormat::Mono => Ok(ctx.extensions().AL_EXT_double()?.AL_FORMAT_MONO_DOUBLE_EXT?),
			ExtDoubleFormat::Stereo => Ok(ctx.extensions().AL_EXT_double()?.AL_FORMAT_STEREO_DOUBLE_EXT?),
		})
	}
}


impl ExtFloat32Format {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtFloat32Format::Mono => Ok(ctx.extensions().AL_EXT_float32()?.AL_FORMAT_MONO_FLOAT32?),
			ExtFloat32Format::Stereo => Ok(ctx.extensions().AL_EXT_float32()?.AL_FORMAT_STEREO_FLOAT32?),
		})
	}
}


impl ExtIma4Format {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtIma4Format::Mono => Ok(ctx.extensions().AL_EXT_IMA4()?.AL_FORMAT_MONO_IMA4?),
			ExtIma4Format::Stereo => Ok(ctx.extensions().AL_EXT_IMA4()?.AL_FORMAT_STEREO_IMA4?),
		})
	}
}


impl ExtMcFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtMcFormat::QuadU8 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD8?),
			ExtMcFormat::QuadI16 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD16?),
			ExtMcFormat::QuadF32 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_QUAD32?),
			ExtMcFormat::RearU8 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_REAR8?),
			ExtMcFormat::RearI16 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_REAR16?),
			ExtMcFormat::RearF32 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_REAR32?),
			ExtMcFormat::Mc51ChnU8 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN8?),
			ExtMcFormat::Mc51ChnI16 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN16?),
			ExtMcFormat::Mc51ChnF32 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_51CHN32?),
			ExtMcFormat::Mc61ChnU8 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN8?),
			ExtMcFormat::Mc61ChnI16 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN16?),
			ExtMcFormat::Mc61ChnF32 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_61CHN32?),
			ExtMcFormat::Mc71ChnU8 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN8?),
			ExtMcFormat::Mc71ChnI16 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN16?),
			ExtMcFormat::Mc71ChnF32 => Ok(ctx.extensions().AL_EXT_MCFORMATS()?.AL_FORMAT_71CHN32?),
		})
	}
}


impl ExtMuLawFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtMuLawFormat::Mono => Ok(ctx.extensions().AL_EXT_MULAW()?.AL_FORMAT_MONO_MULAW_EXT?),
			ExtMuLawFormat::Stereo => Ok(ctx.extensions().AL_EXT_MULAW()?.AL_FORMAT_STEREO_MULAW_EXT?),
		})
	}
}


impl ExtMuLawBFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtMuLawBFormat::B2D => Ok(ctx.extensions().AL_EXT_MULAW_BFORMAT()?.AL_FORMAT_BFORMAT2D_MULAW?),
			ExtMuLawBFormat::B3D => Ok(ctx.extensions().AL_EXT_MULAW_BFORMAT()?.AL_FORMAT_BFORMAT3D_MULAW?),
		})
	}
}


impl ExtMuLawMcFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			ExtMuLawMcFormat::Mono => Ok(ctx.extensions().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_MONO_MULAW?),
			ExtMuLawMcFormat::Stereo => Ok(ctx.extensions().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_STEREO_MULAW?),
			ExtMuLawMcFormat::Quad => Ok(ctx.extensions().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_QUAD_MULAW?),
			ExtMuLawMcFormat::Rear => Ok(ctx.extensions().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_REAR_MULAW?),
			ExtMuLawMcFormat::Mc51Chn => Ok(ctx.extensions().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_51CHN_MULAW?),
			ExtMuLawMcFormat::Mc61Chn => Ok(ctx.extensions().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_61CHN_MULAW?),
			ExtMuLawMcFormat::Mc71Chn => Ok(ctx.extensions().AL_EXT_MULAW_MCFORMATS()?.AL_FORMAT_71CHN_MULAW?),
		})
	}
}


impl SoftMsadpcmFormat {
	pub fn into_raw<'d>(self, ctx: Option<&Context<'d>>) -> AltoResult<sys::ALint> {
		ctx.ok_or(AltoError::AlExtensionNotPresent).and_then(|ctx| match self {
			SoftMsadpcmFormat::Mono => Ok(ctx.extensions().AL_SOFT_MSADPCM()?.AL_FORMAT_MONO_MSADPCM_SOFT?),
			SoftMsadpcmFormat::Stereo => Ok(ctx.extensions().AL_SOFT_MSADPCM()?.AL_FORMAT_STEREO_MSADPCM_SOFT?),
		})
	}
}


unsafe impl SampleFrame for Mono<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::Standard(StandardFormat::MonoU8) }
}
unsafe impl SampleFrame for Mono<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::Standard(StandardFormat::MonoI16) }
}
unsafe impl SampleFrame for Mono<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtFloat32(ExtFloat32Format::Mono) }
}
unsafe impl SampleFrame for Mono<f64> {
	type Sample = f64;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtDouble(ExtDoubleFormat::Mono) }
}
unsafe impl SampleFrame for Mono<ALawSample> {
	type Sample = ALawSample;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtALaw(ExtALawFormat::Mono) }
}
unsafe impl SampleFrame for Mono<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLaw(ExtMuLawFormat::Mono) }
}


unsafe impl SampleFrame for Stereo<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::Standard(StandardFormat::StereoU8) }
}
unsafe impl SampleFrame for Stereo<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::Standard(StandardFormat::StereoI16) }
}
unsafe impl SampleFrame for Stereo<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::ExtFloat32(ExtFloat32Format::Stereo) }
}
unsafe impl SampleFrame for Stereo<f64> {
	type Sample = f64;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::ExtDouble(ExtDoubleFormat::Stereo) }
}
unsafe impl SampleFrame for Stereo<ALawSample> {
	type Sample = ALawSample;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::ExtALaw(ExtALawFormat::Stereo) }
}
unsafe impl SampleFrame for Stereo<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 2 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLaw(ExtMuLawFormat::Stereo) }
}


unsafe impl SampleFrame for McRear<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::RearU8) }
}
unsafe impl SampleFrame for McRear<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::RearI16)  }
}
unsafe impl SampleFrame for McRear<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::RearF32) }
}
unsafe impl SampleFrame for McRear<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 1 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Rear) }
}


unsafe impl SampleFrame for McQuad<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::QuadU8) }
}
unsafe impl SampleFrame for McQuad<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::QuadI16)  }
}
unsafe impl SampleFrame for McQuad<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::QuadF32) }
}
unsafe impl SampleFrame for McQuad<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Quad) }
}


unsafe impl SampleFrame for Mc51Chn<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 6 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc51ChnU8) }
}
unsafe impl SampleFrame for Mc51Chn<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 6 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc51ChnI16)  }
}
unsafe impl SampleFrame for Mc51Chn<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 6 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc51ChnF32) }
}
unsafe impl SampleFrame for Mc51Chn<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 6 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Mc51Chn) }
}


unsafe impl SampleFrame for Mc61Chn<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 7 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc61ChnU8) }
}
unsafe impl SampleFrame for Mc61Chn<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 7 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc61ChnI16)  }
}
unsafe impl SampleFrame for Mc61Chn<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 7 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc61ChnF32) }
}
unsafe impl SampleFrame for Mc61Chn<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 7 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Mc61Chn) }
}


unsafe impl SampleFrame for Mc71Chn<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 8 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc71ChnU8) }
}
unsafe impl SampleFrame for Mc71Chn<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 8 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc71ChnI16)  }
}
unsafe impl SampleFrame for Mc71Chn<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 8 }
	#[inline(always)] fn format() -> Format { Format::ExtMcFormats(ExtMcFormat::Mc71ChnF32) }
}
unsafe impl SampleFrame for Mc71Chn<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 8 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawMcFormats(ExtMuLawMcFormat::Mc71Chn) }
}


unsafe impl SampleFrame for BFormat2D<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 3 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B2DU8) }
}
unsafe impl SampleFrame for BFormat2D<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 3 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B2DI16) }
}
unsafe impl SampleFrame for BFormat2D<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 3 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B2DF32) }
}
unsafe impl SampleFrame for BFormat2D<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 3 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawBFormat(ExtMuLawBFormat::B2D) }
}


unsafe impl SampleFrame for BFormat3D<u8> {
	type Sample = u8;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B3DU8) }
}
unsafe impl SampleFrame for BFormat3D<i16> {
	type Sample = i16;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B3DI16) }
}
unsafe impl SampleFrame for BFormat3D<f32> {
	type Sample = f32;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtBFormat(ExtBFormat::B3DF32) }
}
unsafe impl SampleFrame for BFormat3D<MuLawSample> {
	type Sample = MuLawSample;

	#[inline(always)] fn len() -> usize { 4 }
	#[inline(always)] fn format() -> Format { Format::ExtMuLawBFormat(ExtMuLawBFormat::B3D) }
}


unsafe impl StandardFrame for Mono<u8> { }
unsafe impl StandardFrame for Mono<i16> { }
unsafe impl StandardFrame for Stereo<u8> { }
unsafe impl StandardFrame for Stereo<i16> { }


unsafe impl LoopbackFrame for Mono<u8>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_MONO_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_UNSIGNED_BYTE_SOFT?) }
}
unsafe impl LoopbackFrame for Mono<i16>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_MONO_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_SHORT_SOFT?) }
}
unsafe impl LoopbackFrame for Mono<f32>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_MONO_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_FLOAT_SOFT?) }
}


unsafe impl LoopbackFrame for Stereo<u8>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_STEREO_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_UNSIGNED_BYTE_SOFT?) }
}
unsafe impl LoopbackFrame for Stereo<i16>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_STEREO_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_SHORT_SOFT?) }
}
unsafe impl LoopbackFrame for Stereo<f32>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_STEREO_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_FLOAT_SOFT?) }
}


unsafe impl LoopbackFrame for McQuad<u8>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_QUAD_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_UNSIGNED_BYTE_SOFT?) }
}
unsafe impl LoopbackFrame for McQuad<i16>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_QUAD_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_SHORT_SOFT?) }
}
unsafe impl LoopbackFrame for McQuad<f32>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_QUAD_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_FLOAT_SOFT?) }
}


unsafe impl LoopbackFrame for Mc51Chn<u8>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_5POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_UNSIGNED_BYTE_SOFT?) }
}
unsafe impl LoopbackFrame for Mc51Chn<i16>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_5POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_SHORT_SOFT?) }
}
unsafe impl LoopbackFrame for Mc51Chn<f32>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_5POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_FLOAT_SOFT?) }
}


unsafe impl LoopbackFrame for Mc61Chn<u8>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_6POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_UNSIGNED_BYTE_SOFT?) }
}
unsafe impl LoopbackFrame for Mc61Chn<i16>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_6POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_SHORT_SOFT?) }
}
unsafe impl LoopbackFrame for Mc61Chn<f32>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_6POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_FLOAT_SOFT?) }
}


unsafe impl LoopbackFrame for Mc71Chn<u8>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_7POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_UNSIGNED_BYTE_SOFT?) }
}
unsafe impl LoopbackFrame for Mc71Chn<i16>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_7POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_SHORT_SOFT?) }
}
unsafe impl LoopbackFrame for Mc71Chn<f32>
{
	fn channels(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_7POINT1_SOFT?) }
	fn sample_ty(sl: &ext::ALC_SOFT_loopback) -> AltoResult<sys::ALint> { Ok(sl.ALC_FLOAT_SOFT?) }
}


impl<S> AsBufferData<Mono<S>> for [Mono<S>] where S: Copy, Mono<S>: SampleFrame { fn as_buffer_data(&self) -> &[Mono<S>] { self } }
impl<S> AsBufferData<Mono<S>> for [S] where
	S: Copy,
	Mono<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[Mono<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / Mono::<S>::len()) }
	}
}
impl<S> AsBufferData<Stereo<S>> for [Stereo<S>] where S: Copy, Stereo<S>: SampleFrame { fn as_buffer_data(&self) -> &[Stereo<S>] { self } }
impl<S> AsBufferData<Stereo<S>> for [S] where
	S: Copy,
	Stereo<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[Stereo<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / Stereo::<S>::len()) }
	}
}
impl<S> AsBufferData<McRear<S>> for [McRear<S>] where S: Copy, McRear<S>: SampleFrame { fn as_buffer_data(&self) -> &[McRear<S>] { self } }
impl<S> AsBufferData<McRear<S>> for [S] where
	S: Copy,
	McRear<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[McRear<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / McRear::<S>::len()) }
	}
}
impl<S> AsBufferData<McQuad<S>> for [McQuad<S>] where S: Copy, McQuad<S>: SampleFrame { fn as_buffer_data(&self) -> &[McQuad<S>] { self } }
impl<S> AsBufferData<McQuad<S>> for [S] where
	S: Copy,
	McQuad<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[McQuad<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / McQuad::<S>::len()) }
	}
}
impl<S> AsBufferData<Mc51Chn<S>> for [Mc51Chn<S>] where S: Copy, Mc51Chn<S>: SampleFrame { fn as_buffer_data(&self) -> &[Mc51Chn<S>] { self } }
impl<S> AsBufferData<Mc51Chn<S>> for [S] where
	S: Copy,
	Mc51Chn<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[Mc51Chn<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / Mc51Chn::<S>::len()) }
	}
}
impl<S> AsBufferData<Mc61Chn<S>> for [Mc61Chn<S>] where S: Copy, Mc61Chn<S>: SampleFrame { fn as_buffer_data(&self) -> &[Mc61Chn<S>] { self } }
impl<S> AsBufferData<Mc61Chn<S>> for [S] where
	S: Copy,
	Mc61Chn<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[Mc61Chn<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / Mc61Chn::<S>::len()) }
	}
}
impl<S> AsBufferData<Mc71Chn<S>> for [Mc71Chn<S>] where S: Copy, Mc71Chn<S>: SampleFrame { fn as_buffer_data(&self) -> &[Mc71Chn<S>] { self } }
impl<S> AsBufferData<Mc71Chn<S>> for [S] where
	S: Copy,
	Mc71Chn<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[Mc71Chn<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / Mc71Chn::<S>::len()) }
	}
}
impl<S> AsBufferData<BFormat2D<S>> for [BFormat2D<S>] where S: Copy, BFormat2D<S>: SampleFrame { fn as_buffer_data(&self) -> &[BFormat2D<S>] { self } }
impl<S> AsBufferData<BFormat2D<S>> for [S] where
	S: Copy,
	BFormat2D<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[BFormat2D<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / BFormat2D::<S>::len()) }
	}
}
impl<S> AsBufferData<BFormat3D<S>> for [BFormat3D<S>] where S: Copy, BFormat3D<S>: SampleFrame { fn as_buffer_data(&self) -> &[BFormat3D<S>] { self } }
impl<S> AsBufferData<BFormat3D<S>> for [S] where
	S: Copy,
	BFormat3D<S>: SampleFrame,
{
	fn as_buffer_data(&self) -> &[BFormat3D<S>] {
		unsafe { slice::from_raw_parts(self.as_ptr() as *const _, self.len() / BFormat3D::<S>::len()) }
	}
}


impl<F, T> AsBufferData<F> for T where
	F: SampleFrame,
	T: Deref,
	<T as Deref>::Target: AsBufferData<F>,
{
	fn as_buffer_data(&self) -> &[F] { (**self).as_buffer_data() }
}


impl<S> AsBufferDataMut<Mono<S>> for [Mono<S>] where S: Copy, Mono<S>: SampleFrame { fn as_buffer_data_mut(&mut self) -> &mut [Mono<S>] { self } }
impl<S> AsBufferDataMut<Mono<S>> for [S] where
	S: Copy,
	Mono<S>: SampleFrame,
{
	fn as_buffer_data_mut(&mut self) -> &mut [Mono<S>] {
		unsafe { slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() / Mono::<S>::len()) }
	}
}
impl<S> AsBufferDataMut<Stereo<S>> for [Stereo<S>] where S: Copy, Stereo<S>: SampleFrame { fn as_buffer_data_mut(&mut self) -> &mut [Stereo<S>] { self } }
impl<S> AsBufferDataMut<Stereo<S>> for [S] where
	S: Copy,
	Stereo<S>: SampleFrame,
{
	fn as_buffer_data_mut(&mut self) -> &mut [Stereo<S>] {
		unsafe { slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() / Stereo::<S>::len()) }
	}
}
impl<S> AsBufferDataMut<McRear<S>> for [McRear<S>] where S: Copy, McRear<S>: SampleFrame { fn as_buffer_data_mut(&mut self) -> &mut [McRear<S>] { self } }
impl<S> AsBufferDataMut<McRear<S>> for [S] where
	S: Copy,
	McRear<S>: SampleFrame,
{
	fn as_buffer_data_mut(&mut self) -> &mut [McRear<S>] {
		unsafe { slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() / McRear::<S>::len()) }
	}
}
impl<S> AsBufferDataMut<McQuad<S>> for [McQuad<S>] where S: Copy, McQuad<S>: SampleFrame { fn as_buffer_data_mut(&mut self) -> &mut [McQuad<S>] { self } }
impl<S> AsBufferDataMut<McQuad<S>> for [S] where
	S: Copy,
	McQuad<S>: SampleFrame,
{
	fn as_buffer_data_mut(&mut self) -> &mut [McQuad<S>] {
		unsafe { slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() / McQuad::<S>::len()) }
	}
}
impl<S> AsBufferDataMut<Mc51Chn<S>> for [Mc51Chn<S>] where S: Copy, Mc51Chn<S>: SampleFrame { fn as_buffer_data_mut(&mut self) -> &mut [Mc51Chn<S>] { self } }
impl<S> AsBufferDataMut<Mc51Chn<S>> for [S] where
	S: Copy,
	Mc51Chn<S>: SampleFrame,
{
	fn as_buffer_data_mut(&mut self) -> &mut [Mc51Chn<S>] {
		unsafe { slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() / Mc51Chn::<S>::len()) }
	}
}
impl<S> AsBufferDataMut<Mc61Chn<S>> for [Mc61Chn<S>] where S: Copy, Mc61Chn<S>: SampleFrame { fn as_buffer_data_mut(&mut self) -> &mut [Mc61Chn<S>] { self } }
impl<S> AsBufferDataMut<Mc61Chn<S>> for [S] where
	S: Copy,
	Mc61Chn<S>: SampleFrame,
{
	fn as_buffer_data_mut(&mut self) -> &mut [Mc61Chn<S>] {
		unsafe { slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() / Mc61Chn::<S>::len()) }
	}
}
impl<S> AsBufferDataMut<Mc71Chn<S>> for [Mc71Chn<S>] where S: Copy, Mc71Chn<S>: SampleFrame { fn as_buffer_data_mut(&mut self) -> &mut [Mc71Chn<S>] { self } }
impl<S> AsBufferDataMut<Mc71Chn<S>> for [S] where
	S: Copy,
	Mc71Chn<S>: SampleFrame,
{
	fn as_buffer_data_mut(&mut self) -> &mut [Mc71Chn<S>] {
		unsafe { slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() / Mc71Chn::<S>::len()) }
	}
}
impl<S> AsBufferDataMut<BFormat2D<S>> for [BFormat2D<S>] where S: Copy, BFormat2D<S>: SampleFrame { fn as_buffer_data_mut(&mut self) -> &mut [BFormat2D<S>] { self } }
impl<S> AsBufferDataMut<BFormat2D<S>> for [S] where
	S: Copy,
	BFormat2D<S>: SampleFrame,
{
	fn as_buffer_data_mut(&mut self) -> &mut [BFormat2D<S>] {
		unsafe { slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() / BFormat2D::<S>::len()) }
	}
}
impl<S> AsBufferDataMut<BFormat3D<S>> for [BFormat3D<S>] where S: Copy, BFormat3D<S>: SampleFrame { fn as_buffer_data_mut(&mut self) -> &mut [BFormat3D<S>] { self } }
impl<S> AsBufferDataMut<BFormat3D<S>> for [S] where
	S: Copy,
	BFormat3D<S>: SampleFrame,
{
	fn as_buffer_data_mut(&mut self) -> &mut [BFormat3D<S>] {
		unsafe { slice::from_raw_parts_mut(self.as_mut_ptr() as *mut _, self.len() / BFormat3D::<S>::len()) }
	}
}


impl<F, T> AsBufferDataMut<F> for T where
	F: SampleFrame,
	T: DerefMut,
	<T as Deref>::Target: AsBufferDataMut<F>,
{
	fn as_buffer_data_mut(&mut self) -> &mut [F] { (**self).as_buffer_data_mut() }
}
