using System.Text.Json.Serialization;

namespace PdfSd.Core.Models;

public class DataSlot
{
    [JsonPropertyName("object_id")]
    public uint[] ObjectId { get; set; } = new uint[2];

    [JsonPropertyName("operation_index")]
    public int OperationIndex { get; set; }

    [JsonPropertyName("content")]
    public string Content { get; set; } = string.Empty;

    [JsonPropertyName("is_tagged")]
    public bool IsTagged { get; set; }
}
