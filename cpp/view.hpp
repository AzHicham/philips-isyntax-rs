#pragma once
#include "philips-sys/src/dataenvelopes.rs.h"
#include "PhilipsPixelEngine/pixelengine.hpp"
#include "rust/cxx.h"

struct Rectangle;
struct DimensionsRange;

using View = PixelEngine::View;
using SourceView = PixelEngine::SourceView;
using DisplayView = PixelEngine::DisplayView;
using UserView = PixelEngine::UserView;
using FilterHandle = PixelEngine::FilterHandle;
using Region = PixelEngine::Region;

View& source_view_as_view(SourceView& source_view);

View& display_view_as_view(DisplayView& source_view);

View& user_view_as_view(UserView& source_view);

DimensionsRange dimension_ranges(const View& view, uint32_t level);

void truncation(SourceView& view, bool enabled, bool rounding);

std::shared_ptr<Region> request_region(View& view, const Rectangle &range, bool async_, const std::array<size_t, 3>& backgroundColor);

void fill_buffer(std::shared_ptr<Region> region, rust::Vec<uint8_t>& buffer);
