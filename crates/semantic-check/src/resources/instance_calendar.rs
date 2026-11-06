// SPDX-FileCopyrightText: 2026 Choreoform contributors
// SPDX-License-Identifier: MPL-2.0
use super::*;

pub const CALENDAR_PROFILE: (&str, &str) = (
    "urn:choreoform:calendar-profile:instance-intervals:0.1.0",
    "sha256:2ab8a6a78f2829ababa2d023f6743ac82d7a31acbfca1fb7a03340b7c929f35f",
);

/// Known construction metadata, not an approved/bound live instance calendar.
#[derive(Debug)]
pub struct InstanceCalendar {
    pin: Pin,
    profile: Pin,
    calendar: Calendar,
}
impl InstanceCalendar {
    pub fn pin(&self) -> &Pin {
        &self.pin
    }
    pub fn profile(&self) -> &Pin {
        &self.profile
    }
    pub fn clock(&self) -> &Pin {
        self.calendar.clock()
    }
    pub fn rules(&self) -> &Pin {
        self.calendar.zone_rules()
    }
    pub fn intervals(&self) -> &[(i64, i64)] {
        self.calendar.intervals()
    }
    pub fn contains_coordinate(&self, coordinate: i64) -> bool {
        self.calendar.contains_coordinate(coordinate)
    }
}

impl Resources<'_> {
    /// Verify a known finite coordinate construction. No live activation,
    /// construction approval, instance origin or trusted host facts are supplied.
    pub fn instance_calendar(&mut self, pin: &Pin) -> Result<Arc<InstanceCalendar>> {
        self.budget.tick(0)?;
        if let Some(calendar) = self.instance_calendars.get(pin) {
            return Ok(calendar.clone());
        }
        let v = self.load(pin, "calendar")?;
        let calendar = self.calendar(pin)?;
        require(v["zone"] == "instance-relative", "calendar-zone-profile")?;
        self.clock(calendar.clock())?;
        let rules = self.load(calendar.zone_rules(), "calendar-rules")?;
        fields(&rules, &["kind", "version", "id", "profile", "clock"])?;
        let profile = Pin::decode(&rules["profile"])?;
        require(
            (profile.id.as_str(), profile.revision.as_str()) == CALENDAR_PROFILE,
            "calendar-profile-unsupported",
        )?;
        self.verify(&profile)?;
        let rule_clock = Pin::decode(&rules["clock"])?;
        require(&rule_clock == calendar.clock(), "calendar-rules-clock")?;
        let calendar = Arc::new(InstanceCalendar {
            pin: pin.clone(),
            profile,
            calendar,
        });
        self.instance_calendars
            .insert(pin.clone(), calendar.clone());
        Ok(calendar)
    }
}
