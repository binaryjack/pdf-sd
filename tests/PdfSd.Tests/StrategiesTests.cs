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
        public bool CanHandle(DataSlot slot) => slot.IsTagged;
        public DataSlot Transform(DataSlot slot)
        {
            slot.Content = slot.Content.ToUpper();
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
            new DataSlot { ObjectId = new uint[]{ 1, 0 }, IsTagged = true, Content = "john doe" },
            new DataSlot { ObjectId = new uint[]{ 2, 0 }, IsTagged = false, Content = "30" }
        };

        // Act
        var result = pipeline.Process(slots).ToList();

        // Assert
        Assert.Equal(2, result.Count);
        Assert.Equal("JOHN DOE", result[0].Content);
        Assert.Equal("30", result[1].Content);
    }
}
