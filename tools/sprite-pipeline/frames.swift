// Extract exact video frames as PNGs: frames <video> <outdir> <fps>
import AVFoundation
import AppKit

let args = CommandLine.arguments
let url = URL(fileURLWithPath: args[1])
let outDir = URL(fileURLWithPath: args[2])
let fps = Double(args[3]) ?? 10

let asset = AVURLAsset(url: url)
let gen = AVAssetImageGenerator(asset: asset)
gen.requestedTimeToleranceBefore = .zero
gen.requestedTimeToleranceAfter = .zero
gen.appliesPreferredTrackTransform = true

let duration = CMTimeGetSeconds(asset.duration)
try? FileManager.default.createDirectory(at: outDir, withIntermediateDirectories: true)
var i = 0
var t = 0.0
while t < duration - 0.001 {
    let time = CMTime(seconds: t, preferredTimescale: 600)
    if let cg = try? gen.copyCGImage(at: time, actualTime: nil) {
        let rep = NSBitmapImageRep(cgImage: cg)
        if let data = rep.representation(using: .png, properties: [:]) {
            try data.write(to: outDir.appendingPathComponent(String(format: "f%03d.png", i)))
        }
    }
    i += 1
    t = Double(i) / fps
}
print("duration=\(duration) frames=\(i)")
