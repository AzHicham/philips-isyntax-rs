#include "dataenvelopes.hpp"
#include "philips-sys/src/dataenvelopes.rs.h"

rust::Vec<Rectangle> asRectangles(const DataEnvelopes& data_envelopes) {
    auto rectangles = data_envelopes.asRectangles();

    auto res = rust::Vec<Rectangle>();
    res.reserve(rectangles.size());

    for(const auto &rec: rectangles) {
        res.push_back(Rectangle { rec[0], rec[1], rec[2], rec[3] });
    }
    return res;
}