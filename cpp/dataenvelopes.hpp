#pragma once
#include "PhilipsPixelEngine/pixelengine.hpp"
#include "rust/cxx.h"

struct Rectangle;

using DataEnvelopes = PixelEngine::DataEnvelopes;

rust::Vec<Rectangle> asRectangles(const DataEnvelopes& data_envelopes);
