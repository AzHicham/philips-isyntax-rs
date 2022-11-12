#pragma once
#include <memory>

#include "rust/cxx.h"

#include "PhilipsPixelEngine/pixelengine.hpp"
#include "PhilipsPixelEngine/renderbackend.hpp"
#include "PhilipsPixelEngine/rendercontext.hpp"

using Facade = PixelEngine::ISyntaxFacade;
using View = PixelEngine::View;

struct Range;
struct RegionRequest;
struct Size;
struct ViewWrapper;

std::unique_ptr<PixelEngine> make_pixel_engine(const std::unique_ptr<RenderContext>&,
                                               const std::unique_ptr<RenderBackend>&);

std::unique_ptr<RenderContext> make_render_context();

std::unique_ptr<RenderBackend> make_render_backend();

ViewWrapper open_file(const std::unique_ptr<PixelEngine>&, const std::string&);

float mpp(const ViewWrapper&, rust::Str);

uint32_t level_count(const ViewWrapper&);

Range level_range(const ViewWrapper&, uint32_t);

rust::Vec<Range> envelopes_range(const ViewWrapper&, uint32_t);

void read_region(const std::unique_ptr<PixelEngine>&, const ViewWrapper&, const RegionRequest&, rust::Vec<uint8_t>&,
                 Size&);
