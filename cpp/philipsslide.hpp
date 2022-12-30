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

using ISyntaxFacade = PixelEngine::ISyntaxFacade;
using View = PixelEngine::View;
using SourceView = PixelEngine::SourceView;
using BufferType = PixelEngine::BufferType;
using SubImage = PixelEngine::SubImage;

class PhilipsSlide {
  public:
    PhilipsSlide(rust::Str url);

    void initViews();

    // PixelEngine functions
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

    // file properties
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

    // images properties
    std::string const& pixelTransform(std::string const& subImage) const;
    std::string const& qualityPreset(std::string const& subImage) const;
    size_t quality(std::string const& subImage) const;
    std::string const& compressor(std::string const& subImage) const;
    std::string const& colorspaceTransform(std::string const& subImage) const;
    size_t numTiles(std::string const& subImage) const;
    std::string const& iccProfile(std::string const& subImage) const;
    std::array<double, 9> iccMatrix(std::string const& subImage) const;
    std::vector<uint8_t> const& imageData(std::string const& subImage) const;
    std::string const& lossyImageCompression(std::string const& subImage) const;
    double lossyImageCompressionRatio(std::string const& subImage) const;
    std::string const& lossyImageCompressionMethod(std::string const& subImage) const;
    std::string const& colorLinearity(std::string const& subImage) const;

    // View (over images) functions
    DimensionsRange dimensionRanges(std::string const& subImage, uint32_t level) const;
    std::vector<std::string> const& dimensionNames(std::string const& subImage) const;
    std::vector<std::string> const& dimensionUnits(std::string const& subImage) const;
    std::vector<std::string> const& dimensionTypes(std::string const& subImage) const;
    std::vector<double> const& scale(std::string const& subImage) const;
    std::vector<double> const& origin(std::string const& subImage) const;
    rust::Vec<Rectangle> envelopesAsRectangles(std::string const& subImage, uint32_t level) const;
    uint16_t bitsAllocated(std::string const& subImage) const;
    uint16_t bitsStored(std::string const& subImage) const;
    uint16_t highBit(std::string const& subImage) const;
    uint16_t pixelRepresentation(std::string const& subImage) const;
    uint16_t planarConfiguration(std::string const& subImage) const;
    uint16_t samplesPerPixel(std::string const& subImage) const;
    uint32_t numDerivedLevels(std::string const& subImage) const;
    std::vector<size_t> pixelSize(std::string const& subImage) const;

    void read_region(const RegionRequest& request, rust::Vec<uint8_t>& buffer, Size& image_size) const;

  private:
    std::unique_ptr<RenderContext> _render_context;
    std::unique_ptr<RenderBackend> _render_backend;
    std::unique_ptr<PixelEngine> _pixel_engine;
    ISyntaxFacade& _facade;
    std::map<std::string, View*> _views;

    static const std::string _version; // PixelEngine version
};

class Facade {
  public:
    Facade(ISyntaxFacade& facade);

    void open(rust::Str url) const;
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

    std::unique_ptr<Image> sub_image(std::string const& image_type) const;

  private:
    ISyntaxFacade& _facade;
};

class Image {
  public:
    Image(SubImage& sub_image);

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

  private:
    SubImage& _sub_image;
};

std::unique_ptr<PhilipsSlide> new_(rust::Str url);
