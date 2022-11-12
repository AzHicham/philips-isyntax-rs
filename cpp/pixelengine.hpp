#pragma once
#include <memory>

#include "rust/cxx.h"
#include "PhilipsPixelEngine/pixelengine.hpp"
#include "PhilipsPixelEngine/renderbackend.hpp"
#include "PhilipsPixelEngine/rendercontext.hpp"
#include "region.hpp"

using Facade = PixelEngine::ISyntaxFacade;
using Region = PixelEngine::Region;

std::unique_ptr<PixelEngine> make_pixel_engine(
    const std::unique_ptr<RenderContext>&,
    const std::unique_ptr<RenderBackend>&
);

std::unique_ptr<RenderContext> make_render_context();

std::unique_ptr<RenderBackend> make_render_backend();

std::unique_ptr<std::string> pe_version();

Facade& facade(PixelEngine& pixel_engine, std::string const& name);

void waitAll(PixelEngine& pixel_engine, std::vector<RegionWrapper> const& regions);

std::unique_ptr<std::vector<RegionWrapper>> waitAny(PixelEngine& pixel_engine);

