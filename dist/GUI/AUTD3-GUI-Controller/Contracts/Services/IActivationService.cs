namespace AUTD3_GUI_Controller.Contracts.Services;

public interface IActivationService
{
    Task ActivateAsync(object activationArgs);
}
