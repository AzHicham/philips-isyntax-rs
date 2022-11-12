#pragma once
#include "rust/cxx.h"
#include "PhilipsPixelEngine/pixelengine.hpp"

struct Rectangle;

using DataEnvelopes = PixelEngine::DataEnvelopes;

rust::Vec<Rectangle> asRectangles(const DataEnvelopes& data_envelopes);
