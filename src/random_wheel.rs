/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   random_wheel.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: crenault <crenault@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2015/07/08 21:18:06 by crenault          #+#    #+#             */
/*   Updated: 2015/07/09 14:32:48 by crenault         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

extern crate rand;
extern crate linked_list;
use self::linked_list::LinkedList;
use self::rand::{thread_rng, Rng};

pub struct RandomWheel<'a, T: 'a> {

	sum_proba: f32,
	cards: LinkedList<(f32, &'a T)>
}

impl<'a, T: 'a> RandomWheel<'a, T> {

	pub fn new() -> RandomWheel<'a, T> {

		RandomWheel{
			sum_proba: 0.,
			cards: LinkedList::new()
		}
	}

	/// add an element associated with a probability
	pub fn push(&mut self, proba: f32, data: &'a T) {

		// can do: if even then push back, else push_front
		self.cards.push_back((proba, data));
		self.sum_proba += proba;
	}

	/// return total of luck you add with push
	pub fn sum_proba(&self) -> f32 {

		self.sum_proba
	}

	fn gen_dist(&self) -> f32 {

		rand::thread_rng().gen_range(0., self.sum_proba())
	}

	/// return a ref to the randomly peeked element
	pub fn peek(&self) -> Option<&'a T> {

		let mut rand = self.gen_dist();
		for &(proba, data) in self.cards.iter() {

			rand -= proba;
			if rand <= 0. {
				return Some(data);
			}
		}
		None
	}

	/// Removes a randomly peeked element and return it
	pub fn pop(&mut self) -> Option<&'a T> {

		let mut rand = self.gen_dist();
		let mut chosen_id = None;
		for (index, &(proba, _)) in self.cards.iter().enumerate() {

			rand -= proba;
			if rand <= 0. {
				chosen_id = Some(index);
				break;
			}
		}
		// return and remove data
		if let Some(id) = chosen_id {
			if let Some((_, data)) = self.cards.remove(id) {
				Some(data)
			}
			else { None }
		}
		else { None }
	}
}
