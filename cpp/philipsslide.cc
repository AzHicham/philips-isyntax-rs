#include "philipsslide.hpp"
#include "philips-isyntax-rs/src/philips_slide.rs.h"

const std::string PhilipsSlide::_version = PixelEngine::version();

std::unique_ptr<PhilipsSlide> new_(std::string const& url) { return std::make_unique<PhilipsSlide>(url); }

PhilipsSlide::PhilipsSlide(std::string const& url)
    : _render_context(std::make_unique<SoftwareRenderContext>()),
      _render_backend(std::make_unique<SoftwareRenderBackend>()),
      _pixel_engine(std::make_unique<PixelEngine>(*_render_backend, *_render_context)),
      _facade(_pixel_engine->operator[]("in")) {
    _facade.open(url);
    // init views
    const auto numImages = _facade.numImages();
    for (size_t idx(0); idx < numImages; ++idx) {
        auto& subImage = _facade[idx];
        // eg WSI, MACROIMAGE, LABELIMAGE
        _views.insert(std::pair<std::string, SourceView&>(subImage.imageType(), subImage.sourceView()));
    }
}

std::string const& PhilipsSlide::sdkVersion() const { return _version; }

std::vector<std::string> const& PhilipsSlide::containers() const { return _pixel_engine->containers(); }

std::string const& PhilipsSlide::containerVersion(std::string const& container) const {
    return _pixel_engine->containerVersion(container);
}

std::vector<std::string> const& PhilipsSlide::compressors() const { return _pixel_engine->compressors(); }

std::vector<std::string> const& PhilipsSlide::pixelTransforms() const { return _pixel_engine->pixelTransforms(); }

std::vector<std::string> const& PhilipsSlide::colorspaceTransforms() const {
    return _pixel_engine->colorspaceTransforms();
}

std::vector<std::string> const& PhilipsSlide::qualityPresets() const { return _pixel_engine->qualityPresets(); }

std::vector<std::string> const& PhilipsSlide::supportedFilters() const { return _pixel_engine->supportedFilters(); }

void PhilipsSlide::clientCertificates(std::string const& cert, std::string const& key, std::string const& password) {
    _pixel_engine->clientCertificates(cert, key, password);
}

void PhilipsSlide::certificates(std::string const& path) { _pixel_engine->certificates(path); }

// File properties
size_t PhilipsSlide::numImages() const { return _facade.numImages(); }

std::string const& PhilipsSlide::iSyntaxFileVersion() const { return _facade.iSyntaxFileVersion(); }

std::string const& PhilipsSlide::id() const { return _facade.id(); }

std::string const& PhilipsSlide::barcode() const { return _facade.barcode(); }

std::string const& PhilipsSlide::scannerCalibrationStatus() const { return _facade.scannerCalibrationStatus(); }

std::vector<std::string> const& PhilipsSlide::softwareVersions() const { return _facade.softwareVersions(); }

std::string const& PhilipsSlide::derivationDescription() const { return _facade.derivationDescription(); }

std::string const& PhilipsSlide::acquisitionDateTime() const { return _facade.acquisitionDateTime(); }

std::string const& PhilipsSlide::manufacturer() const { return _facade.manufacturer(); }

std::string const& PhilipsSlide::modelName() const { return _facade.modelName(); }

std::string const& PhilipsSlide::deviceSerialNumber() const { return _facade.deviceSerialNumber(); }

uint16_t PhilipsSlide::scannerRackNumber() const { return _facade.scannerRackNumber(); }

uint16_t PhilipsSlide::scannerSlotNumber() const { return _facade.scannerSlotNumber(); }

std::string const& PhilipsSlide::scannerOperatorId() const { return _facade.scannerOperatorId(); }

uint16_t PhilipsSlide::scannerRackPriority() const { return _facade.scannerRackPriority(); }

std::vector<std::string> const& PhilipsSlide::dateOfLastCalibration() const { return _facade.dateOfLastCalibration(); }

std::vector<std::string> const& PhilipsSlide::timeOfLastCalibration() const { return _facade.timeOfLastCalibration(); }

bool PhilipsSlide::isPhilips() const { return _facade.isPhilips(); }

bool PhilipsSlide::isHamamatsu() const { return _facade.isHamamatsu(); }

bool PhilipsSlide::isUFS() const { return _facade.isUFS(); }

bool PhilipsSlide::isUFSb() const { return _facade.isUFSb(); }

bool PhilipsSlide::isUVS() const { return _facade.isUVS(); }

std::string const& PhilipsSlide::pixelTransform(std::string const& subImage) const {
    return _facade[subImage].pixelTransform();
}

std::string const& PhilipsSlide::qualityPreset(std::string const& subImage) const {
    return _facade[subImage].qualityPreset();
}

size_t PhilipsSlide::quality(std::string const& subImage) const { return _facade[subImage].quality(); }

std::string const& PhilipsSlide::compressor(std::string const& subImage) const {
    return _facade[subImage].compressor();
}

std::string const& PhilipsSlide::colorspaceTransform(std::string const& subImage) const {
    return _facade[subImage].colorspaceTransform();
}

size_t PhilipsSlide::numTiles(std::string const& subImage) const { return _facade[subImage].numTiles(); }

std::string const& PhilipsSlide::iccProfile(std::string const& subImage) const {
    return _facade[subImage].iccProfile();
}

std::array<double, 9> PhilipsSlide::iccMatrix(std::string const& subImage) const {
    return _facade[subImage].iccMatrix();
}

std::vector<uint8_t> const& PhilipsSlide::imageData(std::string const& subImage) const {
    return _facade[subImage].imageData();
}

std::string const& PhilipsSlide::lossyImageCompression(std::string const& subImage) const {
    return _facade[subImage].lossyImageCompression();
}

double PhilipsSlide::lossyImageCompressionRatio(std::string const& subImage) const {
    return _facade[subImage].lossyImageCompressionRatio();
}

std::string const& PhilipsSlide::lossyImageCompressionMethod(std::string const& subImage) const {
    return _facade[subImage].lossyImageCompressionMethod();
}

std::string const& PhilipsSlide::colorLinearity(std::string const& subImage) const {
    return _facade[subImage].colorLinearity();
}

// View (over images) functions
DimensionsRange PhilipsSlide::dimensionRanges(std::string const& subImage, uint32_t level) const {
    const auto ranges = _views.at(subImage).dimensionRanges(level);
    return DimensionsRange{ranges.at(0).at(0), ranges.at(0).at(1), ranges.at(0).at(2),
                           ranges.at(1).at(0), ranges.at(1).at(1), ranges.at(1).at(2)};
}

std::vector<std::string> const& PhilipsSlide::dimensionNames(std::string const& subImage) const {
    return _views.at(subImage).dimensionNames();
}

std::vector<std::string> const& PhilipsSlide::dimensionUnits(std::string const& subImage) const {
    return _views.at(subImage).dimensionUnits();
}

std::vector<std::string> const& PhilipsSlide::dimensionTypes(std::string const& subImage) const {
    return _views.at(subImage).dimensionTypes();
}

std::vector<double> const& PhilipsSlide::scale(std::string const& subImage) const {
    return _views.at(subImage).scale();
}

std::vector<double> const& PhilipsSlide::origin(std::string const& subImage) const {
    return _views.at(subImage).origin();
}

rust::Vec<Rectangle> PhilipsSlide::envelopesAsRectangles(std::string const& subImage, uint32_t level) const {
    auto envelopes_range = _views.at(subImage).dataEnvelopes(level).asRectangles();

    auto res = rust::Vec<Rectangle>();
    res.reserve(envelopes_range.size());

    for (auto& range : envelopes_range) {
        res.push_back(Rectangle{range[0], range[1], range[2], range[3]});
    }
    return res;
}

uint16_t PhilipsSlide::bitsAllocated(std::string const& subImage) const { return _views.at(subImage).bitsAllocated(); }

uint16_t PhilipsSlide::bitsStored(std::string const& subImage) const { return _views.at(subImage).bitsStored(); }

uint16_t PhilipsSlide::highBit(std::string const& subImage) const { return _views.at(subImage).highBit(); }

uint16_t PhilipsSlide::pixelRepresentation(std::string const& subImage) const {
    return _views.at(subImage).pixelRepresentation();
}

uint16_t PhilipsSlide::planarConfiguration(std::string const& subImage) const {
    return _views.at(subImage).planarConfiguration();
}

uint16_t PhilipsSlide::samplesPerPixel(std::string const& subImage) const {
    return _views.at(subImage).samplesPerPixel();
}

size_t PhilipsSlide::numDerivedLevels(std::string const& subImage) const {
    return _views.at(subImage).numDerivedLevels();
}

std::vector<size_t> PhilipsSlide::pixelSize(std::string const& subImage) const {
    return _views.at(subImage).pixelSize();
}

void PhilipsSlide::read_region(const RegionRequest& request, rust::Vec<uint8_t>& buffer, Size& image_size) const {
    auto& view = _views.at("WSI");

    const std::vector<std::vector<std::size_t>> view_range{
        {request.start_x, request.end_x, request.start_y, request.end_y, request.level}};
    auto const& envelopes = view.dataEnvelopes(request.level);

    auto _ = view.requestRegions(view_range, envelopes, false, {254, 254, 254}, BufferType::RGB);
    auto regions = _pixel_engine->waitAny();
    auto region = regions.front();

    // compute image size
    const auto dimension_range = this->dimensionRanges("WSI", request.level);
    const auto& range = region->range();
    image_size.w = 1 + ((range[1] - range[0]) / dimension_range.step_x);
    image_size.h = 1 + ((range[3] - range[2]) / dimension_range.step_y);
    const size_t nb_sub_pixels = image_size.w * image_size.h * 3;

    buffer.reserve(nb_sub_pixels); // RGB pixel
    region->get(buffer.data(), nb_sub_pixels);
}
