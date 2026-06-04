#include <efi.h>
#include <efilib.h>

UINT32 simple_hash(void *data, UINTN size)
{
    UINT8 *p = (UINT8*)data;
    UINT32 h = 5381;

    for (UINTN i = 0; i < size; i++)
        h = ((h << 5) + h) + p[i];

    return h;
}

int verify_integrity(void *kernel, UINTN size, UINT32 expected_hash)
{
    return simple_hash(kernel, size) == expected_hash;
}

EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
    InitializeLib(ImageHandle, SystemTable);

    EFI_FILE_PROTOCOL *root;
    EFI_FILE_PROTOCOL *kernel;

    SystemTable->BootServices->LocateProtocol(
        &gEfiSimpleFileSystemProtocolGuid,
        NULL,
        (void**)&root
    );

    EFI_SIMPLE_FILE_SYSTEM_PROTOCOL *fs = (EFI_SIMPLE_FILE_SYSTEM_PROTOCOL*)root;
    fs->OpenVolume(fs, &root);

    root->Open(root, &kernel, L"kernel.bin", EFI_FILE_MODE_READ, 0);

    UINTN size = 0;
    kernel->GetInfo(kernel, &gEfiFileInfoGuid, &size, NULL);

    void *buffer = AllocatePool(size);
    kernel->Read(kernel, &size, buffer);

    UINT32 expected_hash = 0xA1B2C3D4;

    if (!verify_integrity(buffer, size, expected_hash))
    {
        while (1);
    }

    UINTN map_key = 0;
    SystemTable->BootServices->ExitBootServices(ImageHandle, map_key);

    return 0;
}
