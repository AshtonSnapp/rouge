use self::ops::RangeBounds

impl ubyte:
end

impl byte:
end

impl ushort:
end

impl short:
end

impl uword:
end

impl word:
end

impl ulong:
end

impl long:
end

impl uint:
end

impl int:
end

impl flt:
end

impl dbl:
end

pub struct BoundedUbyte<const RangeBounds<ubyte> R>(ubyte)

pub struct BoundedByte<const RangeBounds<ubyte> R>(byte)

pub struct BoundedUshort<const RangeBounds<ubyte> R>(ushort)

pub struct BoundedShort<const RangeBounds<ubyte> R>(short)

pub struct BoundedUword<const RangeBounds<ubyte> R>(uword)

pub struct BoundedWord<const RangeBounds<ubyte> R>(word)

pub struct BoundedUlong<const RangeBounds<ubyte> R>(ulong)

pub struct BoundedLong<const RangeBounds<ubyte> R>(long)

pub struct BoundedUint<const RangeBounds<ubyte> R>(uint)

pub struct BoundedInt<const RangeBounds<ubyte> R>(int)