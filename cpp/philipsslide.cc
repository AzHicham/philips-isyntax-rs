#include "philipsslide.hpp"
#include "philips-isyntax-rs/src/bindings.rs.h"

const std::string PhilipsEngine::_version = PixelEngine::version();

std::unique_ptr<PhilipsEngine> new_() { return std::make_unique<PhilipsEngine>(); }

PhilipsEngine::PhilipsEngine()
    : _render_context(std::make_unique<SoftwareRenderContext>()),
      _render_backend(std::make_unique<SoftwareRenderBackend>(RenderBackend::ImageFormatType::RGB)),
      _pixel_engine(std::make_unique<PixelEngine>(*_render_backend, *_render_context)) {}

std::string const& PhilipsEngine::sdkVersion() const { return _version; }

std::vector<std::string> const& PhilipsEngine::containers() const { return _pixel_engine->containers(); }

std::string const& PhilipsEngine::containerVersion(std::string const& container) const {
    return _pixel_engine->containerVersion(container);
}

std::vector<std::string> const& PhilipsEngine::compressors() const { return _pixel_engine->compressors(); }

std::vector<std::string> const& PhilipsEngine::pixelTransforms() const { return _pixel_engine->pixelTransforms(); }

std::vector<std::string> const& PhilipsEngine::colorspaceTransforms() const {
    return _pixel_engine->colorspaceTransforms();
}

std::vector<std::string> const& PhilipsEngine::qualityPresets() const { return _pixel_engine->qualityPresets(); }

std::vector<std::string> const& PhilipsEngine::supportedFilters() const { return _pixel_engine->supportedFilters(); }

void PhilipsEngine::clientCertificates(std::string const& cert, std::string const& key, std::string const& password) {
    _pixel_engine->clientCertificates(cert, key, password);
}

void PhilipsEngine::certificates(std::string const& path) { _pixel_engine->certificates(path); }

std::unique_ptr<PixelEngine>& PhilipsEngine::inner() { return _pixel_engine; }

std::unique_ptr<Facade> PhilipsEngine::facade(std::string const& input) const {
    return std::make_unique<Facade>(_pixel_engine->operator[](input));
}
// ------------------------------------

// File properties
Facade::Facade(ISyntaxFacade& facade) : _facade(facade) {}

void Facade::open(rust::Str url, rust::Str container) const {
    std::string _url(url);
    std::string _container(container);
    _facade.open(_url, _container, std::ios::in | std::ios::binary, "");
}

void Facade::close() const { _facade.close(); }

size_t Facade::numImages() const { return _facade.numImages(); }

std::string const& Facade::iSyntaxFileVersion() const { return _facade.iSyntaxFileVersion(); }

std::string const& Facade::id() const { return _facade.id(); }

std::string const& Facade::barcode() const { return _facade.barcode(); }

std::string const& Facade::scannerCalibrationStatus() const { return _facade.scannerCalibrationStatus(); }

std::vector<std::string> const& Facade::softwareVersions() const { return _facade.softwareVersions(); }

std::string const& Facade::derivationDescription() const { return _facade.derivationDescription(); }

std::string const& Facade::acquisitionDateTime() const { return _facade.acquisitionDateTime(); }

std::string const& Facade::manufacturer() const { return _facade.manufacturer(); }

std::string const& Facade::modelName() const { return _facade.modelName(); }

std::string const& Facade::deviceSerialNumber() const { return _facade.deviceSerialNumber(); }

uint16_t Facade::scannerRackNumber() const { return _facade.scannerRackNumber(); }

uint16_t Facade::scannerSlotNumber() const { return _facade.scannerSlotNumber(); }

std::string const& Facade::scannerOperatorId() const { return _facade.scannerOperatorId(); }

uint16_t Facade::scannerRackPriority() const { return _facade.scannerRackPriority(); }

std::vector<std::string> const& Facade::dateOfLastCalibration() const { return _facade.dateOfLastCalibration(); }

std::vector<std::string> const& Facade::timeOfLastCalibration() const { return _facade.timeOfLastCalibration(); }

bool Facade::isPhilips() const { return _facade.isPhilips(); }

bool Facade::isHamamatsu() const { return _facade.isHamamatsu(); }

bool Facade::isUFS() const { return _facade.isUFS(); }

bool Facade::isUFSb() const { return _facade.isUFSb(); }

bool Facade::isUVS() const { return _facade.isUVS(); }

std::unique_ptr<Image> Facade::image(std::string const& image_type) const {
    return std::make_unique<Image>(_facade[image_type]);
}

// ------------------------------------

// Image properties
Image::Image(SubImage& image) : _image(image) {}

std::string const& Image::pixelTransform() const { return _image.pixelTransform(); }

std::string const& Image::qualityPreset() const { return _image.qualityPreset(); }

size_t Image::quality() const { return _image.quality(); }

std::string const& Image::compressor() const { return _image.compressor(); }

std::string const& Image::colorspaceTransform() const { return _image.colorspaceTransform(); }

size_t Image::numTiles() const { return _image.numTiles(); }

std::string const& Image::iccProfile() const { return _image.iccProfile(); }

std::array<double, 9> Image::iccMatrix() const { return _image.iccMatrix(); }

std::vector<uint8_t> const& Image::imageData() const { return _image.imageData(); }

std::string const& Image::lossyImageCompression() const { return _image.lossyImageCompression(); }

double Image::lossyImageCompressionRatio() const { return _image.lossyImageCompressionRatio(); }

std::string const& Image::lossyImageCompressionMethod() const { return _image.lossyImageCompressionMethod(); }

std::string const& Image::colorLinearity() const { return _image.colorLinearity(); }

std::unique_ptr<ImageView> Image::view() const {
    const auto type = _image.imageType();
    auto& source_view = _image.sourceView();
    View& view = static_cast<View&>(source_view); // Should be safe because View is the base class of SourceView

    if (type == "WSI") {
        const auto bitsStored = view.bitsStored();
        // Enable best quality
        const std::map<std::size_t, std::vector<std::size_t>> truncationLevel{{0, {0, 0, 0}}};
        source_view.truncation(false, false, truncationLevel);

        if (bitsStored > 8) {
            PixelEngine::UserView& user_view = source_view.addChainedView();
            user_view.addFilter("Linear16ToSRGB8"); // This Filter converts 9-bit image to 8-bit image.
            view = static_cast<View&>(user_view);   // Should be safe because View is the base class of UserView
        }
    }
    return std::make_unique<ImageView>(view);
}

// ------------------------------------

// View properties
ImageView::ImageView(View& view) : _view(view) {}

DimensionsRange ImageView::dimensionRanges(uint32_t level) const {
    const auto ranges = _view.dimensionRanges(level);
    return DimensionsRange{ranges.at(0).at(0), ranges.at(0).at(1), ranges.at(0).at(2),
                           ranges.at(1).at(0), ranges.at(1).at(1), ranges.at(1).at(2)};
}

std::vector<std::string> const& ImageView::dimensionNames() const { return _view.dimensionNames(); }

std::vector<std::string> const& ImageView::dimensionUnits() const { return _view.dimensionUnits(); }

std::vector<std::string> const& ImageView::dimensionTypes() const { return _view.dimensionTypes(); }

std::vector<double> const& ImageView::scale() const { return _view.scale(); }

std::vector<double> const& ImageView::origin() const { return _view.origin(); }

rust::Vec<Rectangle> ImageView::envelopesAsRects(uint32_t level) const {
    auto envelopes_range = _view.dataEnvelopes(level).asRectangles();

    auto res = rust::Vec<Rectangle>();
    res.reserve(envelopes_range.size());

    for (auto& range : envelopes_range) {
        res.push_back(Rectangle{range.at(0), range.at(1), range.at(2), range.at(3)});
    }
    return res;
}

uint16_t ImageView::bitsAllocated() const { return _view.bitsAllocated(); }

uint16_t ImageView::bitsStored() const { return _view.bitsStored(); }

uint16_t ImageView::highBit() const { return _view.highBit(); }

uint16_t ImageView::pixelRepresentation() const { return _view.pixelRepresentation(); }

uint16_t ImageView::planarConfiguration() const { return _view.planarConfiguration(); }

uint16_t ImageView::samplesPerPixel() const { return _view.samplesPerPixel(); }

uint32_t ImageView::numDerivedLevels() const { return _view.numDerivedLevels(); }

std::vector<size_t> ImageView::pixelSize() const { return _view.pixelSize(); }

void ImageView::read_region(const std::unique_ptr<PhilipsEngine>& engine, const RegionRequest& request,
                            rust::Vec<uint8_t>& buffer, Size& image_size) const {
    const std::vector<std::vector<std::size_t>> view_range{
        {request.roi.start_x, request.roi.end_x, request.roi.start_y, request.roi.end_y, request.level}};
    auto const& envelopes = _view.dataEnvelopes(request.level);
    auto regions = _view.requestRegions(view_range, envelopes, false, {254, 254, 254}, BufferType::RGB);

    auto regions_ready = engine.get()->inner()->waitAny(regions);
    auto region = regions_ready.front();

    // compute image size
    const auto dimension_range = dimensionRanges(request.level);
    const auto& range = region->range();
    image_size.w = 1 + ((range[1] - range[0]) / dimension_range.step_x);
    image_size.h = 1 + ((range[3] - range[2]) / dimension_range.step_y);
    const size_t nb_sub_pixels = image_size.w * image_size.h * 3;

    buffer.reserve(nb_sub_pixels); // RGB pixel
    region->get(buffer.data(), nb_sub_pixels);
}
// ------------------------------------
