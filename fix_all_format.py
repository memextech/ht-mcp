import re
import os

def fix_format_strings_in_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Pattern to match format!("...", var) 
    pattern = r'format!\("([^"]*)\{[^}]*\}([^"]*)",\s*([^)]+)\)'
    
    def replace_func(match):
        pre_text = match.group(1)
        post_text = match.group(2)
        var_expr = match.group(3).strip()
        
        # Handle simple variable names
        if var_expr.isidentifier():
            return f'format!("{pre_text}{{{var_expr}}}{post_text}")'
        # Handle more complex expressions - keep them as is for safety
        else:
            return match.group(0)
    
    # Apply the replacement
    new_content = re.sub(pattern, replace_func, content)
    
    # Handle multiline format! statements
    multiline_pattern = r'format!\s*\(\s*"([^"]*)\{[^}]*\}([^"]*)",\s*\n\s*([^)]+)\s*\)'
    
    def multiline_replace_func(match):
        pre_text = match.group(1)
        post_text = match.group        po  var_expr = match.grou        ip()
                                                         return f'format!("{pre_text}            }}{post_text}")'
        else:
            return match.group(0)
    
    new_content = re.sub(multiline_pattern, multiline_repla   func, new_content, flags=re.MUL    new_content = re.sub(multil != content:
        with open(filepat        with open(filepat        with opntent)
        print(f"Fixed format strings in {fi        print(f"Fixed format stri re        print(f"Fixed format strings in {fi        print(f"Fixx = [
    'src/ht_integration/session_manager.rs',
    'src/mcp/server.rs'
]



  'src/mcp in files_to_fix:
    if os.path.exists(f    if os.path.exists(f   at_strings_in_file(filepath)
    else:
        print(f"File not found: {filepath}")
