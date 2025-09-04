use clap::Parser;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

/// A simple tool to burn subtitles into a video file using ffmpeg.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input video file.
    #[arg()]
    video_in: PathBuf,

    /// Path to the input subtitle file.
    #[arg()]
    sub_in: PathBuf,
}

fn main() {
    let args = Args::parse();

    // 1. Prepare file paths
    let mut video_input_path = args.video_in;
    let mut subtitle_input_path = args.sub_in;

    // 检查文件类型，确保视频和字幕文件没有混淆
    let video_ext = video_input_path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("")
        .to_lowercase();
    let sub_ext = subtitle_input_path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("")
        .to_lowercase();

    // 检查潜在的文件顺序错误，自动调整但不显示消息
    let video_exts = ["mp4", "mkv", "avi", "mov", "webm", "flv", "wmv", "m4v"];
    let sub_exts = ["srt", "ass", "ssa", "vtt", "sub"];

    // 自动检测并调整文件顺序，但不输出任何提示
    if sub_exts.contains(&video_ext.as_str()) && video_exts.contains(&sub_ext.as_str()) {
        std::mem::swap(&mut video_input_path, &mut subtitle_input_path);
    }

    // 2. 让用户交互式选择视频编码器
    println!("\n请选择视频编码器:");
    println!("1. CPU 编码 (libx264) - 通用但较慢");
    println!("2. NVIDIA GPU 编码 (h264_nvenc) - 仅限NVIDIA显卡，速度快");
    println!("3. NVIDIA GPU HEVC编码 (hevc_nvenc) - 更高压缩率，部分播放器可能不支持");
    print!("请输入选项 (1-3): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let (vcodec, is_nvenc) = match choice.trim() {
        "1" => ("libx264", false),
        "2" => ("h264_nvenc", true),
        "3" => ("hevc_nvenc", true),
        _ => {
            println!("无效选择，使用默认编码器 (libx264)");
            ("libx264", false)
        }
    };

    // 3. 让用户选择压制模式和质量
    println!("\n请选择压制模式:");
    println!("1. 高质量保真模式 - 保留原视频最大质量，无码率限制");
    println!("2. 平衡模式 - 在质量和文件大小之间平衡");
    println!("3. 高压缩模式 - 更小的文件大小，适合分享或节省空间");
    print!("请输入选项 (1-3): ");
    io::stdout().flush().unwrap();

    let mut mode_choice = String::new();
    io::stdin().read_line(&mut mode_choice).unwrap();

    let high_quality_mode = match mode_choice.trim() {
        "1" => true,
        "2" | "3" => false,
        _ => {
            println!("无效选择，使用默认高质量保真模式");
            true
        }
    };

    // 根据选择的模式确定默认质量值
    let default_quality = match mode_choice.trim() {
        "1" => {
            if is_nvenc {
                "15"
            } else {
                "18"
            }
        } // 高质量
        "2" => {
            if is_nvenc {
                "20"
            } else {
                "23"
            }
        } // 平衡
        "3" => {
            if is_nvenc {
                "28"
            } else {
                "28"
            }
        } // 高压缩
        _ => {
            if is_nvenc {
                "15"
            } else {
                "18"
            }
        } // 默认高质量
    };

    // 让用户选择质量参数
    let quality_param: String;
    let quality_value: String;

    if is_nvenc {
        // NVENC使用CQ模式
        print!("\n请输入质量值 (CQ, 0-51，值越小质量越高，推荐15-25): ");
        io::stdout().flush().unwrap();

        let mut quality_input = String::new();
        io::stdin().read_line(&mut quality_input).unwrap();

        quality_param = "-cq".to_string();
        quality_value = match quality_input.trim().parse::<u8>() {
            Ok(q) => q.to_string(),
            Err(_) => {
                println!("无效输入，将使用默认质量值 {}", default_quality);
                default_quality.to_string()
            }
        };
    } else {
        // CPU编码使用CRF模式
        print!("\n请输入质量值 (CRF, 0-51，值越小质量越高，推荐18-28): ");
        io::stdout().flush().unwrap();

        let mut quality_input = String::new();
        io::stdin().read_line(&mut quality_input).unwrap();

        quality_param = "-crf".to_string();
        quality_value = match quality_input.trim().parse::<u8>() {
            Ok(q) => q.to_string(),
            Err(_) => {
                println!("无效输入，将使用默认质量值 {}", default_quality);
                default_quality.to_string()
            }
        };
    }

    // 4. 让用户选择编码预设
    println!("\n请选择编码预设:");

    if is_nvenc {
        // NVIDIA预设
        println!("1. p1 - 最高质量，最慢速度");
        println!("2. p2 - 高质量");
        println!("3. p3 - 平衡质量和速度");
        println!("4. p4 - 快速编码");
        println!("5. p5 - 平衡质量和速度的最佳选择");
        println!("6. p6 - 更快速度");
        println!("7. p7 - 最快速度，最低质量");
    } else {
        // CPU预设
        println!("1. veryslow - 最高质量，最慢速度");
        println!("2. slow - 高质量");
        println!("3. medium - 平衡质量和速度");
        println!("4. fast - 快速编码");
        println!("5. veryfast - 更快速度");
        println!("6. ultrafast - 最快速度，最低质量");
    }

    print!("请输入选项: ");
    io::stdout().flush().unwrap();

    let mut preset_choice = String::new();
    io::stdin().read_line(&mut preset_choice).unwrap();

    let preset = if is_nvenc {
        match preset_choice.trim() {
            "1" => "p1",
            "2" => "p2",
            "3" => "p3",
            "4" => "p4",
            "5" => "p5",
            "6" => "p6",
            "7" => "p7",
            _ => {
                println!("无效选择，使用默认预设 p5");
                "p5"
            }
        }
    } else {
        match preset_choice.trim() {
            "1" => "veryslow",
            "2" => "slow",
            "3" => "medium",
            "4" => "fast",
            "5" => "veryfast",
            "6" => "ultrafast",
            _ => {
                println!("无效选择，使用默认预设 medium");
                "medium"
            }
        }
    };

    // 创建输出文件名，加入质量模式信息
    let mut output_path = video_input_path.clone();
    let original_stem = video_input_path.file_stem().unwrap().to_str().unwrap();
    let extension = video_input_path.extension().unwrap().to_str().unwrap();

    // 根据模式添加不同的标记
    let mode_tag = match mode_choice.trim() {
        "1" => "highq",
        "2" => "balanced",
        "3" => "compressed",
        _ => "highq",
    };

    // 检查输出文件扩展名，确保它是视频文件
    let output_extension =
        if ["mp4", "mkv", "avi", "mov", "webm"].contains(&extension.to_lowercase().as_str()) {
            extension
        } else {
            // 如果源文件不是常见视频格式，则默认使用mp4，但不显示警告
            "mp4"
        };

    let new_filename = format!(
        "{}_subbed_{}_{}.{}",
        original_stem, vcodec, mode_tag, output_extension
    );
    output_path.set_file_name(new_filename);
    println!("\n--- 压制信息 ---");
    println!("视频输入: {}", video_input_path.display());
    println!("字幕输入: {}", subtitle_input_path.display());
    println!("视频输出: {}", output_path.display());
    println!("视频编码: {}", vcodec);
    println!("质量参数: {} {}", quality_param, quality_value);
    println!("编码预设: {}", preset);
    println!(
        "压制模式: {}",
        if high_quality_mode {
            "高质量保真模式 (无码率限制)"
        } else if mode_choice.trim() == "2" {
            "平衡模式"
        } else {
            "高压缩模式"
        }
    );

    // 构建ffmpeg命令
    // 字幕路径在Windows上需要特殊处理
    // 需要转义反斜杠和驱动器号冒号
    let escaped_subtitle_path = subtitle_input_path
        .to_str()
        .unwrap()
        .replace('\\', "\\\\")
        .replace(':', "\\:");

    let mut cmd = Command::new("ffmpeg");

    // 首先添加通用选项
    cmd.arg("-hide_banner"); // 隐藏FFmpeg的横幅信息，让输出更简洁

    // 如果使用NVIDIA编码器，添加硬件加速选项
    if is_nvenc {
        cmd.arg("-hwaccel").arg("cuda");
    }

    // 输入文件
    cmd.arg("-i").arg(&video_input_path);

    // 根据视频格式选择合适的滤镜链
    // 对于AV1和其他格式，我们需要确保正确的色彩空间转换
    // 使用更强大的滤镜链以处理各种格式
    cmd.arg("-vf").arg(format!(
        "format=yuv420p,subtitles='{}'",
        escaped_subtitle_path
    ));

    // 视频编码设置
    cmd.arg("-c:v").arg(vcodec);

    // 添加特定于NVENC的参数
    if is_nvenc {
        cmd.arg("-pix_fmt").arg("yuv420p"); // 确保兼容的像素格式
    }

    // 添加预设和质量
    cmd.arg("-preset")
        .arg(preset)
        .arg(quality_param)
        .arg(quality_value);

    // 对于NVENC编码器添加优化参数，但仅在非高质量模式下限制码率
    if is_nvenc {
        // 无论何种模式都使用vbr
        cmd.arg("-rc").arg("vbr"); // 可变比特率模式，通常效果更好

        if !high_quality_mode {
            // 只有在非高质量模式下才限制最大码率
            if mode_choice.trim() == "2" {
                // 平衡模式 - 适中的限制
                cmd.arg("-maxrate:v").arg("30M");
            } else if mode_choice.trim() == "3" {
                // 高压缩模式 - 更严格的限制
                cmd.arg("-maxrate:v").arg("15M");
            }
        }

        // 常量量化模式下不设置目标码率
        cmd.arg("-b:v").arg("0");
    }

    // 复制音频流，不重新编码
    cmd.arg("-c:a").arg("copy");

    cmd.arg(&output_path);

    // 3. Execute the command
    println!("\nExecuting ffmpeg command...");

    // 打印完整命令以便调试
    let command_str: String = cmd
        .get_args()
        .map(|s| s.to_str().unwrap_or(""))
        .collect::<Vec<_>>()
        .join(" ");
    println!("ffmpeg {}", command_str);

    // 执行命令
    let status = cmd
        .status()
        .expect("Failed to execute ffmpeg. Is it installed and in your PATH?");

    if status.success() {
        println!("\nSuccessfully processed video!");
        println!("Output saved to: {}", output_path.display());
    } else {
        eprintln!("\nffmpeg command failed with status: {}", status);
        eprintln!("Please check the ffmpeg output above for errors.");
    }

    // Keep the console window open to see the output
    println!("\nPress Enter to exit...");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
}
