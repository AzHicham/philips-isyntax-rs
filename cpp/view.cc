#include "view.hpp"
#include "philips-sys/src/view.rs.h"

View& source_view_as_view(SourceView& source_view) { return source_view; }

View& display_view_as_view(DisplayView& display_view) { return display_view; }

View& user_view_as_view(UserView& user_view) { return user_view; }

DimensionsRange dimension_ranges(const View& view, uint32_t level) {
    const auto ranges = view.dimensionRanges(level);
    return DimensionsRange{ranges[0][0], ranges[0][1], ranges[0][2], ranges[1][0], ranges[1][1], ranges[1][2]};
}

void truncation(SourceView& view, bool enabled, bool rounding) {
    std::map<std::size_t, std::vector<std::size_t>> truncationLevel;
    truncationLevel[0] = {0, 0, 0};
    view.truncation(enabled, rounding, truncationLevel);
}

std::shared_ptr<Region> request_region(View& view, const Rectangle& range, bool async_,
                                       const std::array<size_t, 3>& bg_color) {
    const std::vector<std::vector<std::size_t>> view_range{{range.x_min, range.x_max, range.y_min, range.y_max}};
    const std::vector<std::size_t> background_color = {bg_color[0], bg_color[1], bg_color[2]};

    auto regions = view.requestRegions(view_range, async_, background_color);

    // We have request only one region
    return regions.front();
}

void get(std::shared_ptr<Region> region, rust::Vec<uint8_t>& buffer) { region->get(buffer.data(), buffer.size()); }
