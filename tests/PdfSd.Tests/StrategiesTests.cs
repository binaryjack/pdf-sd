using System.Collections.Generic;
using System.Linq;
using PdfSd.Core.Models;
using PdfSd.Core.Strategies;
using Xunit;

namespace PdfSd.Tests;

public class StrategiesTests
{
    private class DummyUppercaseStrategy : IDataTransformationStrategy
    {
        public bool CanHandle(DataSlot slot) => slot.Tag == "Name";
        public DataSlot Transform(DataSlot slot)
        {
            slot.OriginalValue = slot.OriginalValue.ToUpper();
            return slot;
        }
    }

    [Fact]
    public void TransformationPipeline_Should_Apply_Correct_Strategy()
    {
        // Arrange
        var pipeline = new TransformationPipeline();
        pipeline.AddStrategy(new DummyUppercaseStrategy());

        var slots = new List<DataSlot>
        {
            new DataSlot { Id = "1", Tag = "Name", OriginalValue = "john doe", Page = 1 },
            new DataSlot { Id = "2", Tag = "Age", OriginalValue = "30", Page = 1 }
        };

        // Act
        var result = pipeline.Process(slots).ToList();

        // Assert
        Assert.Equal(2, result.Count);
        Assert.Equal("JOHN DOE", result[0].OriginalValue);
        Assert.Equal("30", result[1].OriginalValue);
    }
}
