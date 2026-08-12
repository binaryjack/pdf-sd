using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text.Json;
using PdfSd.Core.Models;
using PdfSd.Core.Native;

namespace PdfSd.Core.Engine;

public class PdfManager
{
    public IEnumerable<DataSlot> ExtractStructure(string pdfPath, string structOutPath)
    {
        IntPtr jsonPtr = IntPtr.Zero;
        try
        {
            jsonPtr = PdfEngineNative.extract_pdf_structure(pdfPath, structOutPath);
            if (jsonPtr == IntPtr.Zero)
            {
                throw new Exception("L'extraction a échoué dans le moteur natif.");
            }

            string? json = Marshal.PtrToStringAnsi(jsonPtr);
            if (string.IsNullOrEmpty(json))
            {
                return Array.Empty<DataSlot>();
            }

            return JsonSerializer.Deserialize<List<DataSlot>>(json) ?? new List<DataSlot>();
        }
        finally
        {
            if (jsonPtr != IntPtr.Zero)
            {
                PdfEngineNative.free_rust_string(jsonPtr);
            }
        }
    }

    public void InjectData(string originalPdfPath, IEnumerable<DataSlot> data, string outputPdfPath)
    {
        string json = JsonSerializer.Serialize(data);
        bool success = PdfEngineNative.inject_pdf_data(originalPdfPath, json, outputPdfPath);
        if (!success)
        {
            throw new Exception("L'injection a échoué dans le moteur natif.");
        }
    }
}
