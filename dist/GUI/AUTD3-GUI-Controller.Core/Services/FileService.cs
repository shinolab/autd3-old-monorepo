using System.Text;

using AUTD3_GUI_Controller.Core.Contracts.Services;

using Newtonsoft.Json;

namespace AUTD3_GUI_Controller.Core.Services;

public class FileService : IFileService
{
    public T Read<T>(string folderPath, string fileName)
    {
        var path = Path.Combine(folderPath, fileName);
        if (!File.Exists(path))
        {
            return default;
        }

        var json = File.ReadAllText(path);
        return JsonConvert.DeserializeObject<T>(json);
    }

    public void Save<T>(string folderPath, string fileName, T content)
    {
        if (folderPath != null && !Directory.Exists(folderPath))
        {
            Directory.CreateDirectory(folderPath);
        }

        var fileContent = JsonConvert.SerializeObject(content, Formatting.Indented);
        File.WriteAllText(Path.Combine(folderPath ?? "", fileName), fileContent, Encoding.UTF8);
    }
}
