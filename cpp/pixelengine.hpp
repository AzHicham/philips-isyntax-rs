#pragma once
#include <memory>

#include "PhilipsPixelEngine/pixelengine.hpp"
#include "PhilipsPixelEngine/renderbackend.hpp"
#include "PhilipsPixelEngine/rendercontext.hpp"
#include "rust/cxx.h"

using Facade = PixelEngine::ISyntaxFacade;
using Region = PixelEngine::Region;

std::unique_ptr<PixelEngine> make_pixel_engine(const std::unique_ptr<RenderContext>&,
                                               const std::unique_ptr<RenderBackend>&);

std::unique_ptr<RenderContext> make_render_context();

std::unique_ptr<RenderBackend> make_render_backend();

std::unique_ptr<std::string> version();

Facade& facade(PixelEngine& pixel_engine, std::string const& name);
