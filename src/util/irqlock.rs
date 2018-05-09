// file : irqlock.rs
//
// Copyright (C) 2018  Joshua Barretto <joshua.s.barretto@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use llapi::irq;

// // TODO: Seriously, this whole thing is crap. It should work more like Mutex<T>.

// pub struct IrqLock {
//	 reenable: bool,
// }

// impl IrqLock {
//	 pub fn new() -> IrqLock {
//		 let nlock = IrqLock {
//			 reenable: irq::enabled(),
//		 };
//		 irq::disable();
//		 return nlock;
//	 }
// }

// impl Drop for IrqLock {
//	 fn drop(&mut self) {
//		 if self.reenable {
//			 irq::enable();
//		 } else {
//			 irq::disable();
//		 }
//	 }
// }

use core::{
	cell::UnsafeCell,
	marker::Sync,
	ops::{Drop, Deref, DerefMut},
	default::Default,
};

pub struct IrqLock<T: ?Sized> {
	data: UnsafeCell<T>,
}

pub struct IrqLockGuard<'a, T: ?Sized + 'a> {
	reenable: bool,
	data: &'a mut T,
}

pub struct IrqLockTmp {
	reenable: bool,
}

unsafe impl<T: ?Sized + Send> Sync for IrqLock<T> {}
unsafe impl<T: ?Sized + Send> Send for IrqLock<T> {}

impl IrqLock<()> {
	pub fn temporary() -> IrqLockTmp {
		let tmp = IrqLockTmp {
			reenable: irq::enabled(),
		};
		irq::disable();
		return tmp;
	}
}

impl<T> IrqLock<T> {
	pub const fn new(data: T) -> IrqLock<T> {
		IrqLock {
			data: UnsafeCell::new(data),
		}
	}
}

impl<T: ?Sized> IrqLock<T> {
	pub fn lock(&self) -> IrqLockGuard<T> {
		let guard = IrqLockGuard {
			reenable: irq::enabled(),
			data: unsafe { &mut *self.data.get() },
		};
		irq::disable();
		return guard;
	}
}

impl<'a, T: ?Sized> Drop for IrqLockGuard<'a, T> {
	fn drop(&mut self) {
		if self.reenable {
			irq::enable();
		}
	}
}

impl Drop for IrqLockTmp {
	fn drop(&mut self) {
		if self.reenable {
			irq::enable();
		}
	}
}

impl<T: ?Sized + Default> Default for IrqLock<T> {
	fn default() -> IrqLock<T> {
		IrqLock::new(Default::default())
	}
}

impl<'a, T: ?Sized> Deref for IrqLockGuard<'a, T> {
	type Target = T;
	fn deref<'b>(&'b self) -> &'b T {
		&*self.data
	}
}

impl<'a, T: ?Sized> DerefMut for IrqLockGuard<'a, T> {
	fn deref_mut<'b>(&'b mut self) -> &'b mut T {
		&mut *self.data
	}
}
