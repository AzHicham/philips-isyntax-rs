#pragma once
#include <memory>

#include "rust/cxx.h"
#include "PhilipsPixelEngine/pixelengine.hpp"

using Region = PixelEngine::Region;
using RegionWrapper = std::shared_ptr<Region>;
