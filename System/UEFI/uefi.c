#include <efi.h>
#include <efilib.h>

EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)
{
    InitializeLib(ImageHandle, SystemTable);

    Print(L"[MyOS] Bootloader starting...\n");

    EFI_STATUS status;

    EFI_MEMORY_DESCRIPTOR *memmap = NULL;
    UINTN memmap_size = 0;
    UINTN map_key, desc_size;
    UINT32 desc_version;

    status = uefi_call_wrapper(
        SystemTable->BootServices->GetMemoryMap,
        5,
        &memmap_size,
        memmap,
        &map_key,
        &desc_size,
        &desc_version
    );

    Print(L"[MyOS] Memory map acquired\n");

    EFI_SIMPLE_FILE_SYSTEM_PROTOCOL *fs;

    status = uefi_call_wrapper(
        SystemTable->BootServices->LocateProtocol,
        3,
        &gEfiSimpleFileSystemProtocolGuid,
        NULL,
        (void**)&fs
    );

    Print(L"[MyOS] Filesystem ready\n");

    EFI_FILE_PROTOCOL *root;
    fs->OpenVolume(fs, &root);

    EFI_FILE_PROTOCOL *kernel_file;
    root->Open(root, &kernel_file, L"kernel.elf",
               EFI_FILE_MODE_READ, 0);

    Print(L"[MyOS] Kernel loaded\n");

    status = SystemTable->BootServices->ExitBootServices(ImageHandle, map_key);

    Print(L"[MyOS] Handing control to kernel...\n");

    return EFI_SUCCESS;
}
