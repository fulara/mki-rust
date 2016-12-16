#include "windows.h"

enum Modifiers {
    Modifiers_ALT = 1,
    Modifiers_CTRL = 2,
    Modifiers_SHIFT = 4,
    Modifiers_WIN = 8,
};

int register_hotkey_c(unsigned short key_code, int modifiers, int id) {
    return RegisterHotKey(
                   NULL,
                   id,
                   modifiers,
                   key_code);
}

int wait_for_hotkey_c() {

    MSG msg = {0};
    while (GetMessage(&msg, NULL, 0, 0) != 0)
    {
        if (msg.message == WM_HOTKEY)
        {
            return msg.wParam;
        }
    }

    return 0;
}