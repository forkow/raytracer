use std::{
    num::NonZeroUsize, sync::{mpsc, Arc, Mutex}, time::Instant
};

use image::{ImageBuffer, Rgb};

use crate::{camera::Camera, color::Color, world::World};


pub struct Scene {
    world: World,
    camera: Camera,
}

#[derive(Debug, Clone, Copy)]
pub struct RenderInfo {
    pub width: u32,
    pub height: u32,
    pub max_depth: u32,
    pub gamma: f32,
    pub samples: u32,
}

struct ChunkInfo {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

struct Chunk {
    x: u32,
    y: u32,
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Scene {
    pub fn new(world: World, camera: Camera) -> Self {
        Self { world, camera }
    }

    pub fn render(&self, info: RenderInfo) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let thread_count = std::thread::available_parallelism()
            .map(NonZeroUsize::get)
            .unwrap_or(1);

        let chunks = gen_chunks(info);
        let chunk_count = chunks.len();

        eprintln!("[RENDER] Workers: {}", thread_count);
        eprintln!("[RENDER] Chunks: {}", chunk_count);

        let (completed_tx, completed) = mpsc::channel();
        
        let completed_chunks = std::thread::scope(|scope| {
            let chunks = Arc::new(Mutex::new(chunks));

            for _ in 0..thread_count {
                let chunks = Arc::clone(&chunks);
                let tx = completed_tx.clone();
                scope.spawn(|| {
                    self.spawn_worker(info, chunks, tx);
                });
            }

            drop(completed_tx);

            let mut completed_chunks = Vec::new();

            let start = Instant::now();

            loop {
                eprint!(
                    "[RENDER] Progress: {}/{chunk_count} ({}%)\r",
                    completed_chunks.len(),
                    (completed_chunks.len() * 100) / chunk_count
                );
                if let Ok(chunk) = completed.recv() {
                    completed_chunks.push(chunk);
                } else {
                    break;
                }
            }
            eprintln!();

            let time = start.elapsed().as_secs_f64();

            eprintln!("[RENDER] Rendering took ~{time:.2} seconds");

            completed_chunks
        });

        eprint!("[RENDER] Constructing full image... ");
        println!("{}", completed_chunks.len());

        let mut full_image = ImageBuffer::new(info.width, info.height);

        for chunk in completed_chunks {
            for y in 0..chunk.image.height() {
                for x in 0..chunk.image.width() {
                    full_image[(chunk.x + x, chunk.y + y)] =
                        chunk.image[(x, y)];
                }
            }
        }

        eprintln!("done");

        full_image
    }

    fn spawn_worker(
        &self,
        render_info: RenderInfo,
        chunks: Arc<Mutex<Vec<ChunkInfo>>>,
        completed: mpsc::Sender<Chunk>,
    ) {
        loop {
            let mut chunks = chunks.lock().unwrap();
            let Some(chunk) = chunks.pop() else {
                break;
            };
            drop(chunks);

            let mut image = ImageBuffer::new(chunk.width, chunk.height);

            for y in 0..chunk.height {
                for x in 0..chunk.width {
                    let mut color = Color::black();
                    for _ in 0..render_info.samples {
                        let u = ((chunk.x + x) as f32 + fastrand::f32()) /
                            render_info.width as f32;
                        let v = ((chunk.y + y) as f32 + fastrand::f32()) /
                            render_info.height as f32;

                        color = color +
                            self.camera.trace(
                                u,
                                v,
                                render_info.max_depth,
                                &self.world,
                            );
                    }
                    color = color / render_info.samples as f32;
                    *image.get_pixel_mut(x, y) =
                        color.to_rgb(render_info.gamma);
                }
            }

            completed
                .send(Chunk {
                    x: chunk.x,
                    y: chunk.y,
                    image,
                })
                .unwrap();
        }
    }
}

fn gen_chunks(info: RenderInfo) -> Vec<ChunkInfo> {
    let chunk_width = 128;
    let chunk_height = 128;

    let whole_width_count = info.width / chunk_width;
    let width_rem = info.width % chunk_width;

    let whole_height_count = info.height / chunk_height;
    let height_rem = info.height % chunk_height;

    let whole_count = whole_width_count * whole_height_count;

    let mut chunks = Vec::new();

    for i in 0..whole_count {
        let x = (i % whole_width_count) * chunk_width;
        let y = (i / whole_width_count) * chunk_height;
        chunks.push(ChunkInfo {
            x,
            y,
            width: chunk_width,
            height: chunk_height,
        });
    }

    if height_rem != 0 {
        for i in 0..whole_width_count {
            chunks.push(ChunkInfo {
                x: i * chunk_width,
                y: info.height - height_rem,
                width: chunk_width,
                height: height_rem,
            });
        }
    }

    if width_rem != 0 {
        for i in 0..whole_height_count {
            chunks.push(ChunkInfo {
                x: info.width - width_rem,
                y: i * chunk_height,
                width: width_rem,
                height: chunk_height,
            });
        }
    }

    if width_rem != 0 && height_rem != 0 {
        chunks.push(ChunkInfo {
            x: info.width - width_rem,
            y: info.height - height_rem,
            width: width_rem,
            height: height_rem,
        });
    }

    chunks
}
