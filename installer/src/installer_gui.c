#include <windows.h>

#define BESNICH_DARK    RGB(10, 12, 28)
#define BESNICH_ACCENT  RGB(0, 191, 255)
#define BESNICH_GOLD    RGB(255, 215, 0)

LRESULT CALLBACK WndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam) {
    switch(msg) {
        case WM_CREATE: {
            CreateWindow(TEXT("STATIC"), TEXT("BESNICH OS"),
                WS_VISIBLE | WS_CHILD | SS_CENTER,
                200, 30, 400, 80, hWnd, NULL, NULL, NULL);

            CreateWindow(TEXT("BUTTON"), 
                TEXT("INSTALLA BESNICH"),
                WS_VISIBLE | WS_CHILD | BS_PUSHBUTTON,
                250, 200, 300, 60, hWnd, (HMENU)1, NULL, NULL);

            break;
        }
        case WM_COMMAND: {
            if (LOWORD(wParam) == 1) {
                MessageBox(hWnd,
                    TEXT("Besnich OS verrà installato.\n"
                         "Assicurati di avere:\n"
                         "TPM 2.0\n"
                         "UEFI Secure Boot\n"
                         "20GB spazio libero"),
                    TEXT("Besnich OS - Pronto per l'installazione"),
                    MB_OK | MB_ICONINFORMATION);
                
                InstallBesnich();
            }
            break;
        }
        case WM_DESTROY:
            PostQuitMessage(0);
            break;
    }
    return DefWindowProc(hWnd, msg, wParam, lParam);
}

void InstallBesnich() {
    CreatePartition("BesnichOS", 20480);
    WriteBootloader("bootloader.bin", 0);
    WriteKernel("besnich_kernel.bin", 2048);
    CreateEncryptedFS("BesnichFS", AES_256_GCM);
    
    MessageBox(NULL,
        TEXT("Besnich OS installato con successo!\n"
             "Riavvia il sistema per avviare Besnich."),
        TEXT("Besnich OS - Installazione completata"),
        MB_OK | MB_ICONINFORMATION);
}
