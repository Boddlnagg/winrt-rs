// This tests uses PropertyValue to test static WinRT classes - those classes that lack a default interface
// and thus only provide static methods.

winrt::import!(
    dependencies
        "os"
    modules
        "windows.foundation"
);
use windows::foundation::PropertyValue;

#[test]
fn static_class() -> winrt::Result<()> {
    assert_eq!(
        <PropertyValue as winrt::RuntimeName>::NAME,
        "Windows.Foundation.PropertyValue"
    );

    // TODO: test PropertyValue's methods here

    Ok(())
}
