use crate::desktop::get_current_desktop;
use crate::state::State;
use crate::transformation::Transformation;
use crate::utils::wmctrl;

/// A type representing windows managed by the window manager.
/// An instance is only obtainable through `wmctrl::get_windows()`
///
/// **Note**: Since `wmctrl` fails silently there is no warranty that the actions performed on the window will be successful.
/// This is a flaw in the command line tool itself and not of this crate.
#[derive(Debug)]
pub struct Window {
    id: String,
    desktop: String,
    client_machine: String,
    title: String,
    transformation: Transformation,
    class: String
}

impl Window {
    pub(super) fn new(
        id: String,
        desktop: String,
        client_machine: String,
        title: String,
        transformation: Transformation,
        class: String
    ) -> Window {
        Window {
            id,
            desktop,
            client_machine,
            title,
            transformation,
            class
        }
    }

    fn get(&self) -> String {
        format!("{} -i", self.id)
    }

    /// Set the title of the window
    ///
    /// This method is the equivalent of `wmctrl -r <WIN> -N <STR>`.
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);

        let args = format!("-r {} -N {}", self.get(), title);
        wmctrl(&args);
    }

    /// Set the icon title (short title) of the window
    ///
    /// This method is the equivalent of `wmctrl -r <WIN> -I <STR>`.
    pub fn set_icon_title(&self, title: &str) {
        let args = format!("-r {} -I {}", self.get(), title);
        wmctrl(&args);
    }

    /// Set both the title and icon title of the window
    ///
    /// This method is the equivalent of `wmctrl -r <WIN> -T <STR>`.
    pub fn set_both_title(&mut self, title: &str) {
        self.title = String::from(title);

        let args = format!("-r {} -T {}", self.get(), title);
        wmctrl(&args);
    }

    /// Change the state of the window
    ///
    /// Using this method it's possible for example to make the window maximized, minimized, or fullscreen.
    /// This method is the equivalent of `wmctrl -r <WIN> -b <STARG>`.
    ///
    /// # Examples
    ///
    /// ```
    /// let windows = wmctrl::get_windows();
    /// let win = &windows[0];
    /// // Make the window fullscreen
    /// win.change_state(wmctrl::State::new(wmctrl::Action::Add, wmctrl::Property::Fullscreen));
    /// ```
    pub fn change_state(&self, state: State) {
        let args = format!("-r {} -b {}", self.get(), state);
        wmctrl(&args);
    }

    /// Resize and move the window around the desktop
    ///
    /// This method is the equivalent of `wmctrl -r <WIN> -e <MVARG>`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut windows = wmctrl::get_windows();
    /// let win = &mut windows[0];
    /// // This will move the window to the top left corner and resize it to 960x540
    /// win.transform(wmctrl::Transformation::new(0, 0, 960, 540));
    /// ```
    pub fn transform(&mut self, transformation: Transformation) {
        self.transformation = transformation;

        let args = format!("-r {} -e {}", self.get(), &self.transformation);
        wmctrl(&args);
    }

    /// Move the window to the specified desktop
    ///
    /// This method is the equivalent of `wmctrl -r <WIN> -t <DESK>`.
    pub fn set_desktop(&mut self, desktop: &str) {
        self.desktop = String::from(desktop);

        let args = format!("-r {} -t {}", self.get(), desktop);
        wmctrl(&args);
    }

    /// Move the window to the current desktop and raise it
    ///
    /// This method is the equivalent of `wmctrl -R <WIN>`.
    pub fn activate(&mut self) {
        self.desktop = get_current_desktop();

        let args = format!("-R {}", self.get());
        wmctrl(&args);
    }

    /// Activate the window by switching to its desktop and raising it
    ///
    /// This method is the equivalent of `wmctrl -a <WIN>`.
    pub fn raise(&self) {
        let args = format!("-a {}", self.get());
        wmctrl(&args);
    }

    /// Close the window gracefully
    ///
    /// This method is the equivalent of `wmctrl -c <WIN>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wmctrl::Window;
    ///
    /// // We need to move the window out of the vector so there is no reference left
    /// let win: Window = wmctrl::get_windows().remove(0);
    /// win.close();
    /// ```
    pub fn close(self) {
        let args = format!("-c {}", self.get());
        wmctrl(&args);
    }

    /// Get the title immutably
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn class(&self) -> &String {
        &self.class
    }

    pub fn id(&self) -> &String {
        &self.id
    }
}
