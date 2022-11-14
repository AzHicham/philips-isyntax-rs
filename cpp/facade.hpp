#pragma once
#include <memory>

#include "PhilipsPixelEngine/pixelengine.hpp"
#include "rust/cxx.h"

using Facade = PixelEngine::ISyntaxFacade;
using SubImage = PixelEngine::SubImage;

void open(Facade& facade, std::string const& url, std::string const& containerName, std::string const& cacheName);

SubImage& sub_image(Facade& facade, std::string const& image_type);
