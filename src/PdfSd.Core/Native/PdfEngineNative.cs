using System.Runtime.InteropServices;

namespace PdfSd.Core.Native;

internal static class PdfEngineNative
{
    private const string LibraryName = "core_engine";

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
    public static extern IntPtr extract_pdf_structure(string pdfPath, string structOutPath);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
    public static extern bool inject_pdf_data(string structInPath, string jsonData, string outputPath);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_rust_string(IntPtr ptr);
}
