#pragma once
#include "PhilipsPixelEngine/pixelengine.hpp"
#include "PhilipsPixelEngine/softwarerenderbackend.hpp"
#include "PhilipsPixelEngine/softwarerendercontext.hpp"
#include "rust/cxx.h"
#include <memory>

struct Size;
struct Rectangle;
struct DimensionsRange;
struct RegionRequest;
class Facade;
class Image;
class ImageView;

using ISyntaxFacade = PixelEngine::ISyntaxFacade;
using View = PixelEngine::View;
using SourceView = PixelEngine::SourceView;
using BufferType = PixelEngine::BufferType;
using SubImage = PixelEngine::SubImage;

class PhilipsEngine {
  public:
    PhilipsEngine();

    std::string const& sdkVersion() const;
    std::vector<std::string> const& containers() const;
    std::string const& containerVersion(std::string const& container) const;
    std::vector<std::string> const& compressors() const;
    std::vector<std::string> const& pixelTransforms() const;
    std::vector<std::string> const& colorspaceTransforms() const;
    std::vector<std::string> const& qualityPresets() const;
    std::vector<std::string> const& supportedFilters() const;
    void clientCertificates(std::string const& cert, std::string const& key, std::string const& password);
    void certificates(std::string const& path);
    std::unique_ptr<Facade> facade(std::string const& input) const;
    std::unique_ptr<PixelEngine>& inner();

  private:
    std::unique_ptr<RenderContext> _render_context;
    std::unique_ptr<RenderBackend> _render_backend;
    std::unique_ptr<PixelEngine> _pixel_engine;

    static const std::string _version; // PixelEngine version
};

class Facade {
  public:
    Facade(ISyntaxFacade& facade);

    void open(rust::Str url) const;
    void close() const;
    size_t numImages() const;
    std::string const& iSyntaxFileVersion() const;
    std::string const& id() const;
    std::string const& barcode() const;
    std::string const& scannerCalibrationStatus() const;
    std::vector<std::string> const& softwareVersions() const;
    std::string const& derivationDescription() const;
    std::string const& acquisitionDateTime() const;
    std::string const& manufacturer() const;
    std::string const& modelName() const;
    std::string const& deviceSerialNumber() const;
    uint16_t scannerRackNumber() const;
    uint16_t scannerSlotNumber() const;
    std::string const& scannerOperatorId() const;
    uint16_t scannerRackPriority() const;
    std::vector<std::string> const& dateOfLastCalibration() const;
    std::vector<std::string> const& timeOfLastCalibration() const;
    bool isPhilips() const;
    bool isHamamatsu() const;
    bool isUFS() const;
    bool isUFSb() const;
    bool isUVS() const;
    std::unique_ptr<Image> image(std::string const& image_type) const;

  private:
    ISyntaxFacade& _facade;
};

class Image {
  public:
    Image(SubImage& image);

    std::string const& pixelTransform() const;
    std::string const& qualityPreset() const;
    size_t quality() const;
    std::string const& compressor() const;
    std::string const& colorspaceTransform() const;
    size_t numTiles() const;
    std::string const& iccProfile() const;
    std::array<double, 9> iccMatrix() const;
    std::vector<uint8_t> const& imageData() const;
    std::string const& lossyImageCompression() const;
    double lossyImageCompressionRatio() const;
    std::string const& lossyImageCompressionMethod() const;
    std::string const& colorLinearity() const;
    std::unique_ptr<ImageView> view() const;

  private:
    SubImage& _image;
};

class ImageView {
  public:
    ImageView(View& view);

    DimensionsRange dimensionRanges(uint32_t level) const;
    std::vector<std::string> const& dimensionNames() const;
    std::vector<std::string> const& dimensionUnits() const;
    std::vector<std::string> const& dimensionTypes() const;
    std::vector<double> const& scale() const;
    std::vector<double> const& origin() const;
    rust::Vec<Rectangle> envelopesAsRects(uint32_t level) const;
    uint16_t bitsAllocated() const;
    uint16_t bitsStored() const;
    uint16_t highBit() const;
    uint16_t pixelRepresentation() const;
    uint16_t planarConfiguration() const;
    uint16_t samplesPerPixel() const;
    uint32_t numDerivedLevels() const;
    std::vector<size_t> pixelSize() const;
    void read_region(const std::unique_ptr<PhilipsEngine>& engine, const RegionRequest& request,
                     rust::Vec<uint8_t>& buffer, Size& image_size) const;

  private:
    View& _view;
};

std::unique_ptr<PhilipsEngine> new_();
