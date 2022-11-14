#pragma once
#include "PhilipsPixelEngine/pixelengine.hpp"
#include "rust/cxx.h"

struct Rectangle;

using DataEnvelopes = PixelEngine::DataEnvelopes;

rust::Vec<Rectangle> as_rectangles(const DataEnvelopes& data_envelopes);
