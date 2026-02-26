mod huffman;
mod tree;
use huffman::Huffman;

fn main() {
    let huff = Huffman;
    let data = b"Balloons are popping in the air, tjoho, i like balloons have you played the game bloons? its fun, i think, yeee. Some balloons have helium in them. that makes them fly high in the air. And can also make your voice high. Unfortunatly not higher than that.mushoorms can make you high aswell, not in the same way. Mushrooms, in Mario there are mushrooms. Have you played the game super Mario bros? its fun. YEEE. You can jump on mushrooms. kill them. murder them. and so on. It's a game for kids. maybe it shouldnt be? I dont know. Who am i to judge.YIPIEEEEE. I need to write more stuff. blaaagghhhh, i really want to compress this text, so much into a tiny little text that noone understands. just ones and zeros, but fever than before. it would be bad if it were the same amount of ons and zeros as before. or even worse, imagine if there were more ones and zeros than before. that would be terrible. it actually happend:( I tried the string \"ABCDAB\". that was pretty stupid of me, it basically just blew up the size very stupid. thinking about blowing. you can blow in a balloon to make it bigger. thats a fun fact. have you tried it? well if yo havent, its fine, it probably means you didnt have many friends as a kid. balloons are things you usually have at birthday parties. If you had any of those. Well if not thats also ok. Its just a silly things us human came up with. years, birthdays, hours, seconds, time. what is time? what does it mean? is it real? how do we meassure it? i dont know ask someone else. I dont want to do this anymore. I should really stop. I am become death. To be or not to be. carpe diem. I think, therefore I am. The unexamined life is not worth living. YIHOOO I am done. I managed to reduce my text from 1687 bytes to 1080 bytes, thats pretty much actually. Oh no, it changed, i cant write the result i got. that makes the text longer shit. bloody monkey. now it went from 1900 to 1225. now 1919 to 1250. now 1937 to 1263, i need to stop doing this. I am basically throwing away my life right now. If i predict what will come after i write something, then I could write it! final result is 2159 bytes compressed to 1403 bytes";

    let encoded = huff.encode(data);
    println!("original size: {}", data.len());
    println!("compressed size: {}", encoded.len());

    let decoded = huff.decode(&encoded);
    println!("\n\n decoded text: {}", String::from_utf8(decoded).unwrap());
}
