using System;
using System.IO;
using System.Text.Json;
using System.Threading.Tasks;
using PdfSd.Core.Engine;

namespace PdfSd.Cli;

class Program
{
    static async Task<int> Main(string[] args)
    {
        if (args.Length == 0)
        {
            ShowHelp();
            return 1;
        }

        string command = args[0].ToLowerInvariant();

        try
        {
            if (command == "extract")
            {
                return await HandleExtract(args);
            }
            else if (command == "inject")
            {
                return await HandleInject(args);
            }
            else
            {
                Console.ForegroundColor = ConsoleColor.Red;
                Console.WriteLine($"Commande inconnue : {command}");
                Console.ResetColor();
                ShowHelp();
                return 1;
            }
        }
        catch (Exception ex)
        {
            Console.ForegroundColor = ConsoleColor.Red;
            Console.WriteLine($"Erreur fatale : {ex.Message}");
            Console.ResetColor();
            return 1;
        }
    }

    static async Task<int> HandleExtract(string[] args)
    {
        string? inputPath = GetArgumentValue(args, "-i", "--input");
        string? structPath = GetArgumentValue(args, "-s", "--structure");
        string? dataPath = GetArgumentValue(args, "-d", "--data");

        if (string.IsNullOrEmpty(inputPath) || string.IsNullOrEmpty(structPath) || string.IsNullOrEmpty(dataPath))
        {
            Console.WriteLine("Usage: extract -i <input.pdf> -s <structure.pdf> -d <data.json>");
            return 1;
        }

        if (!File.Exists(inputPath))
        {
            Console.WriteLine($"Erreur: Le fichier d'entrée '{inputPath}' n'existe pas.");
            return 1;
        }

        Console.WriteLine($"Extraction de {Path.GetFileName(inputPath)}...");
        var manager = new PdfManager();
        var slots = manager.ExtractStructure(inputPath, structPath); // Calls FFI to extract data and save skeleton PDF

        var json = JsonSerializer.Serialize(slots, new JsonSerializerOptions { WriteIndented = true });
        await File.WriteAllTextAsync(dataPath, json);
        
        Console.ForegroundColor = ConsoleColor.Green;
        Console.WriteLine($"Extraction terminée avec succès. Fichiers générés: {structPath} et {dataPath}");
        Console.ResetColor();

        return 0;
    }

    static async Task<int> HandleInject(string[] args)
    {
        string? structPath = GetArgumentValue(args, "-s", "--structure");
        string? dataPath = GetArgumentValue(args, "-d", "--data");
        string? outputPath = GetArgumentValue(args, "-o", "--output");

        if (string.IsNullOrEmpty(structPath) || string.IsNullOrEmpty(dataPath) || string.IsNullOrEmpty(outputPath))
        {
            Console.WriteLine("Usage: inject -s <structure.pdf> -d <data.json> -o <output.pdf>");
            return 1;
        }

        if (!File.Exists(structPath))
        {
            Console.WriteLine($"Erreur: Le fichier d'entrée '{structPath}' n'existe pas.");
            return 1;
        }
        if (!File.Exists(dataPath))
        {
            Console.WriteLine($"Erreur: Le fichier de données '{dataPath}' n'existe pas.");
            return 1;
        }

        Console.WriteLine($"Injection des données depuis {Path.GetFileName(dataPath)} vers {Path.GetFileName(structPath)}...");
        
        var json = await File.ReadAllTextAsync(dataPath);
        var slots = JsonSerializer.Deserialize<System.Collections.Generic.List<PdfSd.Core.Models.DataSlot>>(json);

        if (slots == null)
        {
            Console.WriteLine("Erreur: Impossible de parser le fichier JSON des DataSlots.");
            return 1;
        }

        var manager = new PdfManager();
        manager.InjectData(structPath, slots, outputPath);

        Console.ForegroundColor = ConsoleColor.Green;
        Console.WriteLine($"Injection terminée avec succès. Fichier généré: {outputPath}");
        Console.ResetColor();

        return 0;
    }

    static string? GetArgumentValue(string[] args, string shortName, string longName)
    {
        for (int i = 0; i < args.Length - 1; i++)
        {
            if (args[i] == shortName || args[i] == longName)
            {
                return args[i + 1];
            }
        }
        return null;
    }

    static void ShowHelp()
    {
        Console.WriteLine("PDF-SD : Outil d'extraction et d'injection de structure PDF.");
        Console.WriteLine("Commandes:");
        Console.WriteLine("  extract -i <input.pdf> -s <structure.pdf> -d <data.json>");
        Console.WriteLine("  inject -s <structure.pdf> -d <data.json> -o <output.pdf>");
    }
}
