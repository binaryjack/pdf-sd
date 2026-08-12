using System.Collections.Generic;
using PdfSd.Core.Models;

namespace PdfSd.Core.Strategies;

public interface IDataTransformationStrategy
{
    bool CanHandle(DataSlot slot);
    DataSlot Transform(DataSlot slot);
}

public class TransformationPipeline
{
    private readonly List<IDataTransformationStrategy> _strategies = new();

    public void AddStrategy(IDataTransformationStrategy strategy)
    {
        _strategies.Add(strategy);
    }

    public IEnumerable<DataSlot> Process(IEnumerable<DataSlot> slots)
    {
        foreach (var slot in slots)
        {
            var currentSlot = slot;
            foreach (var strategy in _strategies)
            {
                if (strategy.CanHandle(currentSlot))
                {
                    currentSlot = strategy.Transform(currentSlot);
                }
            }
            yield return currentSlot;
        }
    }
}
