// Report magenta/purple-tinted opaque pixels in a sprite PNG: tint <png>
import AppKit

let img = NSImage(contentsOfFile: CommandLine.arguments[1])!
var rect = CGRect(origin: .zero, size: img.size)
let cg = img.cgImage(forProposedRect: &rect, context: nil, hints: nil)!
let w = cg.width, h = cg.height
var b = [UInt8](repeating: 0, count: w * h * 4)
let ctx = CGContext(data: &b, width: w, height: h, bitsPerComponent: 8, bytesPerRow: w * 4,
                    space: CGColorSpaceCreateDeviceRGB(), bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)!
ctx.draw(cg, in: CGRect(x: 0, y: 0, width: w, height: h))

var tinted = 0, edge = 0, interior = 0
var samples: [String: Int] = [:]
for y in 0..<h {
    for x in 0..<w {
        let i = (y * w + x) * 4
        if b[i + 3] == 0 { continue }
        let r = Int(b[i]), g = Int(b[i + 1]), bl = Int(b[i + 2])
        if r - g > 25 && bl - g > 25 && abs(r - bl) < 70 {
            tinted += 1
            var nearClear = false
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = x + dx, ny = y + dy
                if nx < 0 || ny < 0 || nx >= w || ny >= h || b[(ny * w + nx) * 4 + 3] == 0 { nearClear = true }
            }
            if nearClear { edge += 1 } else { interior += 1 }
            let key = "\(r / 16 * 16),\(g / 16 * 16),\(bl / 16 * 16)"
            samples[key, default: 0] += 1
        }
    }
}
print("size \(w)x\(h) tinted \(tinted) (edge \(edge), interior \(interior))")
print(samples.sorted { $0.value > $1.value }.prefix(12).map { "\($0.key):\($0.value)" }.joined(separator: "  "))
