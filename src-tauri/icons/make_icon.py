from PIL import Image
import os

def check_transparent(pixel, target_color=(255, 255, 255), tolerance=10):
    if len(pixel) == 4 and pixel[3] == 0:
        return True
    return all(abs(p - t) <= tolerance for p, t in zip(pixel[:3], target_color))

def remove_white_corners(image_path, output_path):
    print(f"Processing {image_path}...")
    try:
        img = Image.open(image_path).convert("RGBA")
        pixels = img.load()
        width, height = img.size
        
        # We will use a flood fill algorithm from the 4 corners to remove white background
        corners = [(0, 0), (width - 1, 0), (0, height - 1), (width - 1, height - 1)]
        
        # Color to find: white (with some tolerance to handle antialiasing/compression)
        def is_white(x, y):
            p = pixels[x, y]
            return p[0] > 240 and p[1] > 240 and p[2] > 240 and p[3] > 0
            
        stack = []
        for cx, cy in corners:
            if is_white(cx, cy):
                stack.append((cx, cy))
        
        visited = set(stack)
        
        while stack:
            x, y = stack.pop()
            pixels[x, y] = (255, 255, 255, 0)
            
            for dx, dy in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
                nx, ny = x + dx, y + dy
                if 0 <= nx < width and 0 <= ny < height and (nx, ny) not in visited:
                    if is_white(nx, ny):
                        stack.append((nx, ny))
                        visited.add((nx, ny))

        img.save(output_path, "PNG")
        print(f"Successfully saved transparent icon to {output_path}")
    except Exception as e:
        print(f"Error processing image: {e}")

if __name__ == '__main__':
    base_dir = os.path.dirname(os.path.abspath(__file__))
    input_file = os.path.join(base_dir, "icon.png")
    output_file = os.path.join(base_dir, "app-icon.png")
    
    if os.path.exists(input_file):
        remove_white_corners(input_file, output_file)
    else:
        print(f"Error: {input_file} not found.")
