#include "wrapper.hpp"

#include "PhilipsPixelEngine/pixelengine.hpp"
#include "PhilipsPixelEngine/renderbackend.hpp"
#include "PhilipsPixelEngine/rendercontext.hpp"
#include "PhilipsPixelEngine/softwarerenderbackend.hpp"
#include "PhilipsPixelEngine/softwarerendercontext.hpp"
#include "PhilipsPixelEngine/eglrendercontext.hpp"
#include "PhilipsPixelEngine/gles2renderbackend.hpp"

#include "philips-sys/src/bindings.rs.h"

#include <iostream>
#include <algorithm>
#include <functional>
#include <set>
#include <string>
#include <unordered_map>


View* get_view(Facade* facade);

// utilities
using ARRAY_UINT32x3 = std::array<uint32_t, 3>;
using VEC_ARRAY_UINT32x3 = std::vector<ARRAY_UINT32x3>;

struct PatchDimension
{
	std::size_t patchWidth;
	std::size_t patchHeight;
	std::size_t patchSize;
};

PatchDimension getPatchDimension(std::shared_ptr<PixelEngine::Region>& region, VEC_ARRAY_UINT32x3& dimension_ranges)
{
	const std::vector<std::size_t>& range = region->range();

	PatchDimension patch_dimension;

	patch_dimension.patchWidth = 1 + ((range[1] - range[0]) / dimension_ranges[0][1]);
	patch_dimension.patchHeight = 1 + ((range[3] - range[2]) / dimension_ranges[1][1]);

	patch_dimension.patchSize = patch_dimension.patchWidth * patch_dimension.patchHeight * 3;
	return patch_dimension;
}


// API

std::unique_ptr<PixelEngine> make_pixel_engine(
    const std::unique_ptr<RenderContext>& render_context,
    const std::unique_ptr<RenderBackend>& render_backend
)
{
    return std::make_unique<PixelEngine>(*render_backend, *render_context);
}

std::unique_ptr<RenderContext> make_render_context()
{
    return std::make_unique<SoftwareRenderContext>();
}

std::unique_ptr<RenderBackend> make_render_backend()
{
    return std::make_unique<SoftwareRenderBackend>();
}

ViewWrapper open_file(
    const std::unique_ptr<PixelEngine>& pixel_engine,
    const std::string& filename)
{
    auto& facade = (*pixel_engine)["in"];
    // Open with ContainerName = "ficom" to avoid segfault with "read only" mounted disk
    // More info here: https://gitlab.com/BioimageInformaticsGroup/openphi/-/blob/master/openphi/openphi.py
    facade.open(filename, "", std::ios::in | std::ios::binary);
    auto view = get_view(&facade);
    return ViewWrapper{view};
}

View* get_view(Facade* facade)
{
    auto& _facade = *facade;
	PixelEngine::View* view = &_facade["WSI"].sourceView();
	int bitsStored = _facade["WSI"].sourceView().bitsStored();

	PixelEngine::SourceView& sourceView = _facade["WSI"].sourceView();

	if (bitsStored > 8) {
		PixelEngine::UserView& userview = view->addChainedView();
		userview.addFilter("Linear16ToSRGB8"); //This Filter converts 9-bit image to 8-bit image.
		view = &userview;
	}
	else {
		std::map<std::size_t, std::vector<std::size_t>> truncationLevel;
		truncationLevel[0] = { 0, 0, 0 };
		sourceView.truncation(false, false, truncationLevel);
		view = &sourceView;
	}

	return view;
}


float mpp(const ViewWrapper& view_wrapper, rust::Str dimension_name) {
    auto view = view_wrapper.view;
    auto& dimension_names = view->dimensionNames();
    auto iter = std::find(dimension_names.begin(), dimension_names.end(), std::string(dimension_name));
    if (iter != dimension_names.end()) {
        auto index = std::distance(dimension_names.begin(), iter);
        return view->scale()[index];
    }
    else {
        auto message = std::string("Unkown dimension") + std::string(dimension_name);
        throw std::runtime_error(message);
    }
}

uint32_t level_count(const ViewWrapper& view_wrapper)
{
    return view_wrapper.view->numDerivedLevels() + 1;
}

Range level_range(const ViewWrapper& view_wrapper, uint32_t level) {
    auto ranges = view_wrapper.view->dimensionRanges(level);
    return Range {
        ranges[0][0],
        ranges[0][2],
        ranges[1][0],
        ranges[1][2],
    };
}

rust::Vec<Range> envelopes_range(const ViewWrapper& view_wrapper, uint32_t level) {
    auto envelopes_range = view_wrapper.view->dataEnvelopes(level).asRectangles();

    auto res = rust::Vec<Range>();
    res.reserve(envelopes_range.size());

    for (auto &range : envelopes_range) {
        res.push_back(Range {
            range[0],
            range[1],
            range[2],
            range[3]
        });
    }
    return res;
}



void read_region(
    const std::unique_ptr<PixelEngine>& pixel_engine,
    const ViewWrapper& view_wrapper,
    const RegionRequest& request,
    rust::Vec<uint8_t>& buffer,
    Size& image_size
) {
    auto enable_async_rendering = request._async;
    auto bg_color = request.background_color;
    size_t r = bg_color[0]; size_t g = bg_color[1]; size_t b = bg_color[2];
    const std::vector<std::size_t> background_color = { r, g, b };

    const std::vector<std::vector<std::size_t>> view_range = {{
        request.start_x,
        request.end_x,
        request.start_y,
        request.end_y,
        request.level
    }};

    auto view = view_wrapper.view;
    auto const& envelopes = view->dataEnvelopes(request.level);

    // Vec<(start, step, end)> over dimensions (ie x, y, z...)
    auto dimension_ranges = view->dimensionRanges(request.level);

    // We ask only 1 region even if the Philips API can handle multiple parallel request
    auto regions = view->requestRegions(view_range, envelopes, enable_async_rendering, background_color);
    auto regions_ready = pixel_engine->waitAny();

    // Again we only ask for 1 patch/tile
    auto region = regions.front();

    auto patch_dimension = getPatchDimension(region, dimension_ranges);
    image_size.w = patch_dimension.patchWidth;
    image_size.h = patch_dimension.patchHeight;
    buffer.reserve(patch_dimension.patchSize);
    region->get(buffer.data(), patch_dimension.patchSize);
}
