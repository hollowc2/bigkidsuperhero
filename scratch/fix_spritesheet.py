from PIL import Image
import os

def fix_sprite():
    input_path = 'assets/bridget_sprite.png'
    output_path = 'assets/bridget_sprite.png'
    backup_path = 'assets/bridget_sprite_opaque_backup.png'

    # Create backup if not already there
    if not os.path.exists(backup_path):
        os.rename(input_path, backup_path)
        input_path = backup_path

    img = Image.open(input_path).convert('RGBA')
    datas = img.getdata()

    new_data = []
    # Targeted grey color found at (0,0)
    bg_color = (164, 166, 165)
    
    for item in datas:
        # If pixel matches the grey color, make it transparent
        # Using a small tolerance just in case
        if abs(item[0] - bg_color[0]) < 2 and \
           abs(item[1] - bg_color[1]) < 2 and \
           abs(item[2] - bg_color[2]) < 2:
            new_data.append((255, 255, 255, 0))
        else:
            new_data.append(item)

    img.putdata(new_data)
    
    # Crop to multi of 8 rows of 149
    # 896x1192
    img = img.crop((0, 0, 896, 1192))
    
    img.save(output_path, 'PNG')
    print(f"Processed sprite saved to {output_path}")

if __name__ == '__main__':
    fix_sprite()
